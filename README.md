# Contract Module

The Contract module provides functionality for the runtime to deploy and execute WebAssembly smart-contracts.

- [`contract::Trait`](./trait.Trait.html)
- [`Call`](./enum.Call.html)

## Overview

This module extends accounts based on the `Currency` trait to have smart-contract functionality. It can
be used with other modules that implement accounts based on `Currency`. These "smart-contract accounts"
have the ability to instantiate smart-contracts and make calls to other contract and non-contract accounts.

The smart-contract code is stored once in a `code_cache`, and later retrievable via its `code_hash`.
This means that multiple smart-contracts can be instantiated from the same `code_cache`, without replicating
the code each time.

When a smart-contract is called, its associated code is retrieved via the code hash and gets executed.
This call can alter the storage entries of the smart-contract account, instantiate new smart-contracts,
or call other smart-contracts.

Finally, when an account is reaped, its associated code and storage of the smart-contract account
will also be deleted.

### Gas

Senders must specify a gas limit with every call, as all instructions invoked by the smart-contract require gas.
Unused gas is refunded after the call, regardless of the execution outcome.

If the gas limit is reached, then all calls and state changes (including balance transfers) are only
reverted at the current call's contract level. For example, if contract A calls B and B runs out of gas mid-call,
then all of B's calls are reverted. Assuming correct error handling by contract A, A's other calls and state
changes still persist.

### Notable Scenarios

Contract call failures are not always cascading. When failures occur in a sub-call, they do not "bubble up",
and the call will only revert at the specific contract level. For example, if contract A calls contract B, and B
fails, A can decide how to handle that failure, either proceeding or reverting A's changes.

## Interface

### Dispatchable functions

* `put_code` - Stores the given binary Wasm code into the chain's storage and returns its `code_hash`.
* `instantiate` - Deploys a new contract from the given `code_hash`, optionally transferring some balance.
This instantiates a new smart contract account and calls its contract deploy handler to
initialize the contract.
* `call` - Makes a call to an account, optionally transferring some balance.

## Usage

The Contract module is a work in progress. The following examples show how this Contract module
can be used to instantiate and call contracts.

* [`ink`](https://github.com/paritytech/ink) is
an [`eDSL`](https://wiki.haskell.org/Embedded_domain_specific_language) that enables writing
WebAssembly based smart contracts in the Rust programming language. This is a work in progress.

## Related Modules

* [Balances](../pallet_balances/index.html)

## License

---
Copyright 2020 Maciej Baj.

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0
    
---
