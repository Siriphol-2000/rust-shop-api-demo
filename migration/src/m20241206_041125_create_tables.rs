use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create the `users` table
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(User::Email)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(User::PasswordHash).string().not_null())
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT NOW()".to_owned()),
                    )
                    .col(
                        ColumnDef::new(User::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT NOW()".to_owned()),
                    )
                    .to_owned(),
            )
            .await?;

        // Create the `products` table
        manager
            .create_table(
                Table::create()
                    .table(Product::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Product::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Product::Name).string().not_null())
                    .col(ColumnDef::new(Product::Description).text().null())
                    .col(ColumnDef::new(Product::Price).decimal().not_null())
                    .col(
                        ColumnDef::new(Product::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT NOW()".to_owned()),
                    )
                    .col(
                        ColumnDef::new(Product::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT NOW()".to_owned()),
                    )
                    .to_owned(),
            )
            .await?;

        // Create the `carts` table
        manager
            .create_table(
                Table::create()
                    .table(Cart::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Cart::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Cart::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Cart::Table, Cart::UserId)
                            .to(User::Table, User::Id),
                    )
                    .col(
                        ColumnDef::new(Cart::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT NOW()".to_owned()),
                    )
                    .col(
                        ColumnDef::new(Cart::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT NOW()".to_owned()),
                    )
                    .to_owned(),
            )
            .await?;

        // Create the `cart_items` table
        manager
            .create_table(
                Table::create()
                    .table(CartItem::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CartItem::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(CartItem::CartId).integer().not_null())
                    .col(ColumnDef::new(CartItem::ProductId).integer().not_null())
                    .col(ColumnDef::new(CartItem::Quantity).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(CartItem::Table, CartItem::CartId)
                            .to(Cart::Table, Cart::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(CartItem::Table, CartItem::ProductId)
                            .to(Product::Table, Product::Id),
                    )
                    .col(
                        ColumnDef::new(CartItem::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT NOW()".to_owned()),
                    )
                    .col(
                        ColumnDef::new(CartItem::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT NOW()".to_owned()),
                    )
                    .to_owned(),
            )
            .await?;

        // Create the `orders` table
        manager
            .create_table(
                Table::create()
                    .table(Order::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Order::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Order::UserId).integer().not_null())
                    .col(ColumnDef::new(Order::TotalAmount).decimal().not_null())
                    .col(ColumnDef::new(Order::PaymentStatus).string().not_null())
                    .col(
                        ColumnDef::new(Order::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT NOW()".to_owned()),
                    )
                    .col(
                        ColumnDef::new(Order::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT NOW()".to_owned()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Order::Table, Order::UserId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // Create the `order_items` table
        manager
            .create_table(
                Table::create()
                    .table(OrderItem::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(OrderItem::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(OrderItem::OrderId).integer().not_null())
                    .col(ColumnDef::new(OrderItem::ProductId).integer().not_null())
                    .col(ColumnDef::new(OrderItem::Quantity).integer().not_null())
                    .col(ColumnDef::new(OrderItem::Price).decimal().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(OrderItem::Table, OrderItem::OrderId)
                            .to(Order::Table, Order::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(OrderItem::Table, OrderItem::ProductId)
                            .to(Product::Table, Product::Id),
                    )
                    .col(
                        ColumnDef::new(OrderItem::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT NOW()".to_owned()),
                    )
                    .col(
                        ColumnDef::new(OrderItem::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT NOW()".to_owned()),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(OrderItem::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Order::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(CartItem::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Cart::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Product::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(User::Table).to_owned()).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Email,
    PasswordHash,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Product {
    Table,
    Id,
    Name,
    Description,
    Price,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Cart {
    Table,
    Id,
    UserId,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum CartItem {
    Table,
    Id,
    CartId,
    ProductId,
    Quantity,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Order {
    Table,
    Id,
    UserId,
    TotalAmount,
    PaymentStatus,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum OrderItem {
    Table,
    Id,
    OrderId,
    ProductId,
    Quantity,
    Price,
    CreatedAt,
    UpdatedAt,
}
