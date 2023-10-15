use anyhow::{Error, Result};
use postgres::{Transaction, types::Type};
use crate::section1::entity::ProductCategory;

use super::{entity::Product, repository::Repository};

pub struct ProductRepository<'a, 'b> (pub &'a mut Transaction<'b>);
impl Repository<Product, i64, u64> for ProductRepository<'_, '_> {
    fn select_all(&mut self) -> Result<Vec<Product>> {
        let sql = "SELECT id, name, price, category_id FROM product";
        let rows = self.0.query(sql, &[])?;
        let mut products = Vec::<Product>::new();

        for row in rows.iter() {
            products.push(Product::new(
                row.get("id"),
                row.get("name"),
                row.get("price"),
                row.get("category_id"),
                None
            ));
        }

        Ok(products)
    }

    fn select_by_id(&mut self, id: i64) -> Result<Product> {
        let sql = "SELECT id, name, price, category_id FROM product WHERE id = $1";
        let stmt = self.0.prepare_typed(sql, &[Type::INT8])?;

        let result = self.0.query_opt(&stmt, &[&id])?;
        match result {
            Some(row) => Ok(Product::new(
                row.get("id"),
                row.get("name"),
                row.get("price"),
                row.get("category_id"),
                None
            )),
            None => Err(Error::msg(format!("Specified product id {} is not found", id)))
        }
    }

    fn insert(&mut self, row: Product) -> Result<u64> {
        let sql = "INSERT INTO product (name, price, category_id) VALUES ($1, $2, $3)";
        let stmt = self.0.prepare_typed(
            sql,
                &[Type::VARCHAR, Type::INT4, Type::INT8]
        )?;

        let result = self.0.execute(
            &stmt,
            &[
                row.get_name(), row.get_price(), row.get_category_id()
            ]
        )?;
        Ok(result)
    }
}

impl ProductRepository<'_, '_>  {
    pub fn select_by_id_join_product(&mut self, id: i64) -> Result<ProductCategory> {
        let sql = format!("{} {}",
                          "SELECT c.id AS c_id, c.name AS c_name, p.id, p.name, p.price, p.category_id",
                          "FROM product_category c JOIN product p ON c.id = p.category_id WHERE c.id = $1"
        );
        let stmt = self.0.prepare_typed(sql.as_str(), &[Type::INT8])?;
        let rows = self.0.query(&stmt, &[&id])?;
        if rows.is_empty() {
            return Err(Error::msg(format!("Specified product category id {} is not found", id)));
        }

        let mut product_category = ProductCategory::new(
            rows[0].get("c_id"),
            rows[0].get("c_name"),
            None
        );

        let mut products = Vec::<Product>::new();
        for row in rows.iter() {
            products.push(Product::new(
                row.get("id"),
                row.get("name"),
                row.get("price"),
                row.get("category_id"),
                Some(product_category.clone())
            ));
        }
        product_category.set_products(Some(products));
        Ok(product_category)
    }

    pub fn avg_by_price(&mut self) -> Result<f64> {
        let sql = "SELECT CAST(AVG(price) AS FLOAT) AS price_avg FROM product";
        let row = self.0.query_one(sql, &[])?;
        let result = row.get("price_avg");
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use anyhow::Result;
    use postgres::Client;

    use crate::section1::params::ConnectParams;
    use crate::section1::connect::PostgresSampleClient;
    use crate::section1::transaction::TransactionUtil;

    fn create_client() -> Client {
        let params = ConnectParams::new(
            "localhost".to_string(),
            5432,
            "product".to_string(),
            "admin".to_string(),
            "password".to_string()
        );

        PostgresSampleClient::simple_connect(params).unwrap()
    }

    #[test]
    fn select_by_id() -> Result<()> {
        let mut client = create_client();

        let mut tx = TransactionUtil::start(&mut client, true)?;

        let mut repository = ProductRepository(&mut tx);

        let result = repository.select_by_id(1);
        match result {
            Ok(product) => println!("{:?}", product.to_string()),
            Err(e) => println!("{}", e.to_string())
        };

        // not exists
        let result = repository.select_by_id(1000);
        match result {
            Ok(_) => assert!(false),
            Err(err) => println!("{:?}", err.to_string())
        };

        Ok(())
    }

    #[test]
    fn insert() -> Result<()> {
        let mut client = create_client();
        let mut tx = TransactionUtil::start(&mut client, false)?;
        let mut repository = ProductRepository(&mut tx);

        let product = Product::new(0, "油性ボールペン(黒)".to_string(), 200, 1, None);
        let result = repository.insert(product);
        match result {
            Ok(count) => {
                let _ = TransactionUtil::commit(tx);
                assert_eq!(count, 1)
            },
            Err(err) => println!("insert: {}", err.to_string())
        };

        Ok(())
    }

    #[test]
    fn select_by_id_join_product() -> Result<()> {
        let mut client = create_client();
        let mut tx = TransactionUtil::start(&mut client, true)?;
        let mut repository = ProductRepository(&mut tx);

        let result = repository.select_by_id_join_product(1);
        match result {
            Ok(product_category) => println!("{:?}", product_category.to_string()),
            Err(err) => println!("select_by_id_join_product: {}", err.to_string())
        };

        Ok(())
    }

    #[test]
    fn avg_by_price() -> Result<()> {
        let mut client = create_client();
        let mut tx = TransactionUtil::start(&mut client, true)?;
        let mut repository = ProductRepository(&mut tx);

        let result = repository.avg_by_price();
        match result {
            Ok(avg) => println!("avg_by_price: {}", avg),
            Err(err) => println!("avg_by_price: {}", err.to_string())
        };

        Ok(())
    }
}
