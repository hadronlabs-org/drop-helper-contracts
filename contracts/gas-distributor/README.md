## Drop Gas Distributor

This contract's purpose is to sustain balances on provided neutron addresses (ICQ/IBC relayers, some of Drop contracts for IBC fees etc.)

### Execute Methods:

#### `distribute`

**Description**: Method's purpose is to distribute tokens that given smart contract possess among provided target balances. Make sure to send enough `untrn` tokens on instantiated smart contract before executing it, otherwise you get an error InsufficientFunds.

**Parameters**: No

**Permissionless**: Yes

#### `set_target_balances`

**Description**: Method's purpose is to set new target balances in order to make it part of upcoming `distribution` call. If target's real balance is 100 untrn and `update_options.threshold_balance` is 101 untrn, then it sends `update_options.target_balance` - `current_balance`. Note that balances are strings and counted in untrn

**Parameters**:

```json
{
    "set_target_balances": {
        "target_balances": [
            {
                "address": string,
                "update_options": {
                    "threshold_balance": string,
                    "target_balance": string
                }
            }
        ]
    }
}
```

- `add_target_balances.add_target_balances.address`: neutron address where this constract supposed to send tokens.
- `add_target_balances.add_target_balances.update_options.threshold_balance`: lower threshold of balance for given neutron address. When it's reached, it sends `update_options.target_balance` - `current_balance`
- `add_target_balances.add_target_balances.update_options.target_balance` amount of tokens this contract should reach on specified target

**Permissionless**: Yes

#### `withdraw_tokens`

**Description**: Method's purpose is to withdraw remaining funds from given contract.

**Parameters**:

```json
{
  "withdraw_tokens": {
    "recepient": null | string,
    "amount": null | string
  }
}
```

- `withdraw_tokens.recepient`: recepient who supposed to get remaining amount of tokens on this contract. If this field wasn't provided then ownership will be assigned to sender's address
- `withdraw_tokens.amount`: amount of tokens that `withdraw_tokens.recepient` will get after execution. If this field wasn't provided then it takes contract's current balance

**Permissionless**: No

### Query Methods:

#### `target_balances`

**Description**: Get all registered target balances

**Parameters**: No

#### `target_balance`

**Description**: Get info about specific registered target balance

**Parameters**:

```json
{
  "target_balance": {
    "address": string,
  }
}
```

- `target_balance.address`: neutron address of registered target balance you want to know information about

### Instantiate Message

**Description**: `initial_target_balances` values are very same as in `add_target_balances`. If owner was not provided then ownership will be assigned to sender's address

```json
{
  "owner": null | string,
  "initial_target_balances": [
    {
      "address": string,
      "update_options": {
        "target_balance": string,
        "update_value": string
      }
    }
  ]
}
```
