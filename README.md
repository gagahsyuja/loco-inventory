```sh
                      ▄     ▀
                                 ▀  ▄
                  ▄       ▀     ▄  ▄ ▄▀
                                    ▄ ▀▄▄
                        ▄     ▀    ▀  ▀▄▀█▄
                                          ▀█▄
▄▄▄▄▄▄▄  ▄▄▄▄▄▄▄▄▄   ▄▄▄▄▄▄▄▄▄▄▄ ▄▄▄▄▄▄▄▄▄ ▀▀█
 ██████  █████   ███ █████   ███ █████   ███ ▀█
 ██████  █████   ███ █████   ▀▀▀ █████   ███ ▄█▄
 ██████  █████   ███ █████       █████   ███ ████▄
 ██████  █████   ███ █████   ▄▄▄ █████   ███ █████
 ██████  █████   ███  ████   ███ █████   ███ ████▀
   ▀▀▀██▄ ▀▀▀▀▀▀▀▀▀▀  ▀▀▀▀▀▀▀▀▀▀  ▀▀▀▀▀▀▀▀▀▀ ██▀
       ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀
                https://loco.rs
```
# Welcome to Loco Inventory :train:

This app uses:
- **loco** -- backend framework
- **postgres** -- database
- **docker** -- containerization

## How to Achieve This
1. Install `loco` and `sea-orm-cli`
```sh
cargo install loco
cargo install sea-orm-cli # Needed for DB

```
2. Create a new loco app
```sh
❯ loco new
✔ ❯ App name? · loco-inventory
✔ ❯ What would you like to build? · Rest API (with DB and user auth)
✔ ❯ Select a DB Provider · Postgres
✔ ❯ Select your background worker type · Async (in-process tokio async tasks)

🚂 Loco app generated successfully in:
loco-inventory/

- database: You've selected `postgres` as your DB provider (you should have a postgres instance to connect to)
```

3. Create scaffolds that includes model and controller for all tables
```sh
# admins table
cargo loco generate scaffold admins name:text description:text --api

# categories table with relation to admins
cargo loco generate scaffold categories name:text description:text admins:references:created_by --api

# suppliers table with relation to admins
cargo loco generate scaffold suppliers name:text contact_info:text admins:references:created_by --api

# items table with relation to categories and suppliers
cargo loco generate scaffold items name:text description:text price:decimal quantity:int \
    categories:references:category_id suppliers:references:supplier_id admins:references:created_by --api
```

4. Create a `.env` for our environment variables

5. Creates seeders located in `src/fixtures/`

6. Implement all database operations

7. Generate a docker deployment and modify it
```sh
cargo loco generate deployment --kind docker
```

8. Create a `docker-compose.yml` and configure it to have loco and postgres container

9. All done

## Run With Docker

```sh
docker compose up --build --detach
```

## API Documentation

1. Create and read item

`POST /api/items`

`GET /api/items`

2. Create and read category

`POST /api/categories`

`GET /api/categories`

3. Create and read supplier

`POST /api/suppliers`

`GET /api/suppliers`

4. Show items stock reports 

`GET /api/reports`

5. Show list of items with stocks below a certain number

`GET /api/items/filter-by-quantity/{number}`

6. Show reports of items with a certain category

`GET /api/items/filter-by-category/{id}`

7. Show summary on a certain category

`GET /api/summaries/category`

`GET /api/summaries/category/{id}`

8. Show items summary that supplied by a certain supplier

`GET /api/summaries/supplier}`

`GET /api/summaries/supplier/{id}`

9. Show summary for everything

`GET /api/summaries`

