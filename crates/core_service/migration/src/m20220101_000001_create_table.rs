use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
    .get_connection()
    .execute_unprepared(
        "CREATE TYPE order_status AS ENUM ('created', 'paid', 'shipped', 'completed', 'canceled')"
    )
    .await?;

manager
    .get_connection()
    .execute_unprepared(
        "CREATE TYPE payment_status AS ENUM ('pending', 'succeeded', 'failed', 'refunded')"
    )
    .await?;

        manager
            .create_table(
                Table::create()
                    .table(Coffee::Table)
                    .if_not_exists()
                    .col(pk_auto(Coffee::Id))
                    .col(string(Coffee::Brand).not_null())
                    .col(string(Coffee::Sort).not_null())
                    .col(string(Coffee::OriginCountry).not_null())
                    .col(string(Coffee::Processing).not_null())
                    .col(string(Coffee::RoastLevel).not_null())
                    .col(string(Coffee::FlavourNotes).not_null())
                    .col(integer(Coffee::Sweetness).not_null().default(5))
                    .col(integer(Coffee::Acidity).not_null().default(5))
                    .col(integer(Coffee::Bitterness).not_null().default(5))
                    .check(Expr::col(Coffee::Acidity).between(1, 10))
                    .check(Expr::col(Coffee::Sweetness).between(1, 10))
                    .check(Expr::col(Coffee::Bitterness).between(1, 10))
                    .col(ColumnDef::new(Coffee::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(boolean(Coffee::IsActive).not_null().default(true))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Product::Table)
                    .if_not_exists()
                    .col(pk_auto(Product::Id))
                    .col(integer(Product::CoffeeId).not_null())
                    .col(decimal(Product::Price).not_null())
                    .col(integer(Product::WeightGrams).not_null())
                    .col(integer(Product::Stock).not_null())
                    .col(string(Product::Grinding).not_null())
                    .col(ColumnDef::new(Product::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(string(Product::ImageUrl).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-product-coffee_id")
                            .from(Product::Table, Product::CoffeeId)
                            .to(Coffee::Table, Coffee::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PickupPoint::Table)
                    .if_not_exists()
                    .col(pk_auto(PickupPoint::Id))
                    .col(string(PickupPoint::Name).not_null())
                    .col(string(PickupPoint::City).not_null())
                    .col(string(PickupPoint::Street).not_null())
                    .col(integer(PickupPoint::Number).not_null())
                    .col(string(PickupPoint::Phone).not_null())
                    .col(string(PickupPoint::WorkingHours).not_null())
                    .col(boolean(PickupPoint::IsActive).not_null().default(true))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Order::Table)
                    .if_not_exists()
                    .col(pk_auto(Order::Id))
                    .col(integer(Order::PickupPointId).not_null())
                    .col(ColumnDef::new(Order::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Order::Status).custom(Alias::new("order_status")).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-order-pickup_point_id")
                            .from(Order::Table, Order::PickupPointId)
                            .to(PickupPoint::Table, PickupPoint::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(OrderItem::Table)
                    .if_not_exists()
                    .col(pk_auto(OrderItem::Id))
                    .col(integer(OrderItem::OrderId).not_null())
                    .col(integer(OrderItem::ProductId).not_null())
                    .col(integer(OrderItem::Quantity).not_null())
                    .col(decimal(OrderItem::Price).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-order_item-order_id")
                            .from(OrderItem::Table, OrderItem::OrderId)
                            .to(Order::Table, Order::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-order_item-product_id")
                            .from(OrderItem::Table, OrderItem::ProductId)
                            .to(Product::Table, Product::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Payment::Table)
                    .if_not_exists()
                    .col(pk_auto(Payment::Id))
                    .col(integer(Payment::OrderId).not_null())
                    .col(string(Payment::Provider).not_null())
                    .col(string(Payment::ProviderTxnId).not_null().unique_key())
                    .col(decimal(Payment::Amount).not_null())
                    .col(string(Payment::Currency).not_null())
                    .col(ColumnDef::new(Payment::Status).custom(Alias::new("payment_status")).not_null())
                    .col(ColumnDef::new(Payment::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Payment::PaidAt).timestamp_with_time_zone())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-payment-order_id")
                            .from(Payment::Table, Payment::OrderId)
                            .to(Order::Table, Order::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Payment::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(OrderItem::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Order::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(PickupPoint::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Product::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Coffee::Table).to_owned()).await?;
        manager.get_connection().execute_unprepared("DROP TYPE IF EXISTS payment_status").await?;
        manager.get_connection().execute_unprepared("DROP TYPE IF EXISTS order_status").await?;
        Ok(())
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
    FlavourNotes,
    Sweetness,
    Acidity,
    Bitterness,
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
    ImageUrl,
}

#[derive(DeriveIden)]
enum PickupPoint {
    Table,
    Id,
    Name,
    City,
    Street,
    Number,
    Phone,
    WorkingHours,
    IsActive,
}

#[derive(DeriveIden)]
enum Order {
    Table,
    Id,
    CreatedAt,
    Status,
    PickupPointId,
}

#[derive(DeriveIden)]
enum OrderItem {
    Table,
    Id,
    OrderId,
    ProductId,
    Quantity,
    Price,
}

#[derive(DeriveIden)]
enum Payment {
    Table,
    Id,
    OrderId,
    Provider,
    ProviderTxnId,
    Amount,
    Currency,
    Status,
    CreatedAt,
    PaidAt,
}

