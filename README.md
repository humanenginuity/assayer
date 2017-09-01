# Assayer

### a•SEY•er, n.
1. A detailed report of the findings in analysis of a substance
2. An unfortunate person whose job it was to taste test royal food, in case of poison
3. Static (compile-time) validation library for Rust

## Motivation
Frequently, programs need to process untrusted input.  Before doing so, it is important to ensure that the values being
processed are, in fact, valid for the purpose for which they are intended.

Values representing the hour of the day, the day of the month, a person's age, a URL, phone number, and so on.
Regardless of the underlying data type (integer, string, custom struct, etc.) some values are valid, and some are not.

As a validation library, Assayer allows you to build efficient, highly cohesive code by concentrating your custom
validation logic to one place--your validator.  You can create any number of validators for each of the different types
in your application requiring validation.

## How to Use
### Quickstart  
- Define your custom validator (eg. for non-empty strings) by implementing `ValidatorRef` and/or `ValidatorMutRef`
and/or `Validator` depending on the move/copy/mutability semantics you want.  Here is an example using `ValidatorRef`:
```rust
extern crate assayer;
use assayer::{ValidatorRef, ValidatorMutRef, Validator};  //Ok to import just the one(s) you want to use
use assayer::{MethodSyntaxValidatorRef, MethodSyntaxValidatorMutRef, MethodSyntaxValidator}; //ditto
use assayer::Error as AssayerError;

struct NonEmptyString;  //A dummy custom type to validate

impl ValidatorRef<String> for NonEmptyString {
    fn validate_ref(input: &String) -> std::result::Result<&String, AssayerError> {
        Ok(input)
            .and_then(|input| match !input.is_empty() {
                true => Ok(input),
                false => Err(AssayerError::ValueEmpty("<Validation failure message here>".to_string())),
            })

        // Chain as many additional .and_then() clauses as you need to validate your input.
        // Each validation clause in the .and_then() chain must return:
        //   * Ok(input) if the value passes that validation step or
        //   * Err(String) if the value fails that validation step.
        // A clause in a Result::and_then() chain which returns Err() will terminate further validation processing
        // and the returned Err() will be returned to the caller.

        //.and_then(|input| ...test #2...)
        //...
        //.and_then(|input| ...test #n...)
    }
}

```
To call the the validator, use either the function- or method-call syntax illustrated below:
```rust
    let empty_string = String::new();
    let non_empty_string = "Not an empty string".to_string();

    let function_call_syntax_result = String::validate_ref::<NonEmptyString>(&empty_string);
    let method_call_syntax_result = (&non_empty_string).validate_ref::<NonEmptyString>();

    assert_eq!(function_call_syntax_result.err(),
               Some(AssayerError::ValueEmpty("<Validation failure message here>".to_string())));
    assert_eq!(method_call_syntax_result.ok(), Some(&non_empty_string));
```
### FAQ
#### What are some of Assayer's design goals?
##### • Lightweight, Zero Dependencies
Languages have long been heading down the path of pulling in other "modules" and building on top of them (the Javascript
community is particularly notable in this regard).  Rust's Cargo and crate system brings easy dependencies to Rust,
which we fully welcome.  At the same time, we are fans of beautiful, maintainable, predictable, small focuse code.  When
less code can do more, we think that kind of efficiency gain is particularly elegant.  So the library ended up being a
grand total of six trait definitions, plus three blanket impls (which is how `MethodSyntaxValidator<SomeType>` gets defined
automatically when you as the user implement only `Validator<SomeType>`), and no dependencies.
you as the user of the library )

##### • Facilitating High Cohesion, Single-Responsiblity, Don't Repeat Yourself Principles
A validator allows you to factor out all your checking into pre-conditions all in one place.  When all the code in a
routine is focused on achieving the same goal, you get high cohesion, which as been shown to be very beneficial to the
reliability and maintainability of a given codebase.

By focusing only validation code into a validator, the validator becomes responsible for just one thing: validation.
The single-responsibility principle helps to keep the code from growing into a "god object", which, if you've ever had
the pleasure of working on one, we're sure you'll agree you don't want to go back any time soon!

##### • Static (compile-time) validator
As a static validator, the validation chain you create is able to fully benefit from the compiler optimizers.  A dynamic
validator using, for example, the builder pattern at run-time cannot realize these benefits, because the validation chain
does not exist as compile time.

We thought about various use-cases, and could not think of a drawback for static validators.  If you come across a
compelling, please let us know!

#### What is the difference between function-call syntax and method-call syntax?
From a code perspective, none.  (For the tests we've run under Rust v1.18.0, the compiler yields 100% identical assembly
instructions.)

From a personal perspective, you may be more comfortable with one style or the other, or it may be
convenient to use one particular style or the other under specific circumstances.  The choice is yours.

#### When should I implement `Validator` vs. `ValidatorRef` vs. `ValidatorMutRef`?
Implementing `Validator` means that your validator will *consume* the value being passed by the caller:
```rust
struct NonEmptyStringValidator;

impl Validator<String> for NonEmptyString {
    fn validate(input: String) -> std::result::Result<String, AssayerError> {
        Ok(input)
            .and_then(|input| match !input.is_empty() {
                true => Ok(input),
                false => Err(AssayerError::ValueEmpty("<Validation failure message here>".to_string())),
            })
    }
}

fn main() {
    let input = "Not an empty string".to_string();
    let result = input.validate::<NonEmptyString>();
    assert_eq!(result.ok(), Some(input)); //Oops!  input has been moved!
}
```
If this is undesirable, either:
	- the caller can use `input.clone().validate::<NotEmptyStringValidator>()`, or
	- you may choose to implement `ValidateRef` and use `(&input).validate_ref::<NotEmptyStringValidator>()`.

The case for implementing `ValidatorMutRef` is somewhat rare and arguably a violation of the Single Responsibility
Principle.  Whether you consider it to be or not, it can be used to 'fix up' an incoming value before it is used by your
post-validation code (you might want to pad or round a valid input value before it is used, for example).  Here is how it is used:
```rust
impl ValidatorMutRef<String> for NonEmptyString {
    fn validate_mut_ref(input: &mut String) -> std::result::Result<&mut String, AssayerError> {
        Ok(input)
            .and_then(|input| match !input.is_empty() {
                true => { input.push_str("!"); Ok(input) }, //proof validate_mut_ref can mutate the input value
                false => Err(AssayerError::ValueEmpty("<Validation failure message here>".to_string())),
            })
    }
}

fn main() {
    let mut input = "Not an empty string".to_string();
    let mut expected_result_inner = "Not an empty string!".to_string();
    let expected_result = Some(&mut expected_result_inner);

    let result = (&mut input).validate_mut_ref::<NonEmptyString>();
    assert_eq!(result.ok(), expected_result);
}
```

Finally, if you simply want all three implementations to "just work" whenever you implement one, implement ValidateRef as per above according to your needs, and "blanket implementations" for ValidateMutRef and Validate as follows:

```rust
impl ValidatorMutRef<String> for NonEmptyStringValidator {
    fn validate_mut_ref(input: &mut String) -> Result<&mut String> {
        Ok(input)
            .and_then(|input| match (&*input).validate_ref::<NonEmptyStringValidator>() {
                Ok(_) => Ok(input),
                Err(err) => Err(err),
            })
    }
}

impl Validator<String> for NonEmptyStringValidator {
    fn validate(input: String) -> Result<String> {
        Ok(input)
            .and_then(|input| match (&input).validate_ref::<NonEmptyStringValidator>() {
                Ok(_) => Ok(input),
                Err(err) => Err(err),
            })
    }
}
```