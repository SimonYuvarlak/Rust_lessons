# Task: Create a Simple Banking Application

#### Overview

Develop a simple banking application in Rust that utilizes OOP principles like encapsulation, polymorphism, and traits. Your application will manage bank accounts, allowing for basic operations like creating an account, depositing, and withdrawing money.

#### Requirements

1. **Define a `BankAccount` Struct**:

   - Fields: `account_number` (String), `balance` (private, f64).
   - Implement a constructor `new` to create a new account with an initial balance.
   - Implement methods `deposit` and `withdraw`. The `withdraw` method should return a `Result<f64, String>` indicating success or a descriptive error message.

2. **Implement Encapsulation**:

   - Ensure that the balance cannot be accessed or modified directly from outside the struct.

3. **Create a `Bank` Struct**:

   - Fields: `accounts` (Vec<BankAccount>).
   - Implement methods to add a new account and to get an account by its number.

4. **Implement a `Display` Trait for `BankAccount`**:

   - This should return a string representation of the bank account (excluding the balance for privacy).

5. **Create a Trait `Transaction` with Methods `credit` and `debit`**:

   - Implement this trait for `BankAccount`.

6. **User Interface**:
   - In your `main.rs`, create a user interface that allows users to:
     - Create a new bank account.
     - Deposit money into an account.
     - Withdraw money from an account.
     - Display account details.

#### Steps

1. **Setup**: Create a new Rust project and define the `BankAccount` and `Bank` structs in `lib.rs`.

2. **Implement Methods and Traits**: Write the necessary methods and traits as per the requirements.

3. **User Interaction**: In `main.rs`, write code to interact with the user, allowing them to utilize the banking functionalities.

4. **Testing**: Test each functionality separately to ensure correctness.

5. **Challenge**: As an extra challenge, implement a feature to transfer funds between accounts, ensuring proper error handling for cases like insufficient funds.

This task is designed to provide a practical application of OOP concepts in Rust, focusing on encapsulation, traits, and user interaction. Upon completion, you will have a basic yet functional banking application demonstrating OOP principles in Rust.
