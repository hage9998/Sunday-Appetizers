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

table! {
  use diesel::sql_types::*;

  /// Adresses.
  pate.adresses (id) {
      /// Identifier of the address.
      #[sql_name = "address_id"]
      id -> Integer,
      /// Address street
      address_street -> Text,
      /// Address number
      address_number -> Integer,
      /// Address district
      address_district -> Text,
      /// Address postcode
      zip_postcode -> Text,
      /// City name
      city_name -> Text,
      /// State name
      state_name -> Text,
      /// Country name
      country_name -> Text,
      /// Address details
      other_address_details -> Text,
  }
}
