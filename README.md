# Codechef Inventory Management
(Project made as a part of the recruitment process for Codechef.)

## Technologies being used:
- Language: Rust
- Framework: Rocket
- Database: In-Memory

## FEATURES:
1. Get all present items in the inventory in JSON format.
2. Add new items to the inventory.
3. Check for duplicate items before adding them to the database.
4. Session based authentication.
5. Update items in your inventory
6. Delete items in your inventory
7. User login info is securely hashed

## TODO:
- [x] implement user sign up and log in using ~~JWT~~ session-based auth
- [x] verified user auth functionality using POSTMAN
- [x] Implement Get specific ID, Update and Delete functionalities
- [x] Hash the user password
- [ ] Organize the README
- [ ] Add database to the project
- [ ] Implement fuzzy searching for items **[If time permits]** 
- [ ] Rate Limiting middleware **[If time permits]** 