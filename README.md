# Simple DBMS (Data Base Managment System) by Emil Hultcrantz

Simple DBMS capable of hanfling a single 

## **How to run**

* Clone repo
* Navigate into repo directory `emilhul-hash`
* Run `cargo run -- ` followed by arguments in the repo directory. For help on which arguments to use run `cargo run -- help` or check documentation.

### **Documentation**

**USAGE:** `cargo run -- <PATH> <COMMAND> <ARGUMENTS>`

| Description | Command | Arguments |
| ------- | ----------- | --------- |
| Delete a value by a given key | `delete` | `Key`: The key to remove the value at |
| Get a value by a given key | `get` | `Key`: The key to get the value at |
| Print this message or the help of the given subcommand(s) | `help` | |
| Insert a given value at a given key | `insert` | `Key`: The key to insert the value at, `Value`: The value to insert | 
| Prints table | `print` | |
