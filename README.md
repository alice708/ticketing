# ticketing
A ticketing system like ticketmaster to practice creating a distributed system

#
Start with
docker compose up -d

# Decisions
Rust sql library: sqlx vs diesel.
Diesel offers compile time checking of your sql query as you build it (which would be good for long lived codebases) and is faster. SQLx let's you write in SQL, which I already know, so it will be faster to start with. I will use SQLx, and if performance becomes a problem I can switch to Diesel and run perf tests to see the difference.