table! {
  use diesel::sql_types::*;

  /// Customers.
  pate.customers (id) {
      /// Identifier of the customer.
      #[sql_name = "customer_id"]
      id -> Uuid,
      /// Customer first name.
      first_name -> Text,
      /// Customer last name.
      last_name -> Text,
      /// Customer phone number.
      customer_phone -> Text,
      /// Customer email.
      customer_email -> Text,
  }
}

table! {
  use diesel::sql_types::*;
  use crate::models::product::product_data::*;

  /// Products.
  pate.products (id) {
      /// Identifier of the product.
      #[sql_name = "product_id"]
      id -> Integer,
      /// Product price.
      product_price -> Numeric,
      /// Product name.
      product_name -> Text,
      /// Product type.
      product_type -> TypesProdMapping,
  }
}
