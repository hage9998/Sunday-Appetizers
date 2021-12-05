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
