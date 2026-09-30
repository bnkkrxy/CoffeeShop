use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(pk_auto(Post::Id))
                    .col(string(Post::Title))
                    .col(string(Post::Text))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Coffee {
    Table,
    Id,
    Brand,
    Sort,
    OriginCountry,
    Processing,
    RoastLevel,
    CreatedAt,
    IsActive,
}

#[derive(DeriveIden)]
enum Product {
    Table,
    Id,
    CoffeeId,
    Price,
    WeightGrams,
    Stock,
    Grinding,
    CreatedAt,
}

#[derive(DeriveIden)]
enum FlavorNotes {
    Table,
    Id,
    Name,
    Category,
}

#[derive(DeriveIden)]
enum CoffeeFlavorNotes {
    Table,
    CoffeeId,
    FlavourNoteId,
}

#[derive(DeriveIden)]
enum Order {
    Table,
    Id,
    UserId,
    CreatedAt,
    Status,
    AddressId,
    DeliveryType
}

#[derive(DeriveIden)]
enum Address {
    Table,
    Id,
    City,
    Street,
    Number,
    Entrance,
    Flat,
    Floor,
}

/*
OrderItem
id
order_id
product_id
quantity
price

User
id
name
email
phone
password_hash
role
created_at

UserAddress
id
user_id
city
street
number
entrance
flat
floor

Payment
id
order_id
provider
provider_txn_id
amount
currency
status
created_at
paid_at

ProductImage
id
product_id 
url 
sort_order 
is_main 
 */
