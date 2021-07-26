# Rust contract upgrading a game player

Create a subaccount for this:

```bash
export NEAR_ACCT=YOUR_ACCOUNT_NAME.testnet 
near create-account player.$NEAR_ACCT --masterAccount $NEAR_ACCT
```

Or remake if need be:
```bash
near delete player.$NEAR_ACCT $NEAR_ACCT
near create-account player.$NEAR_ACCT --masterAccount $NEAR_ACCT
```

Build and deploy:
```bash
./build.sh
near deploy player.$NEAR_ACCT --wasmFile res/upgrade_player_example.wasm
```

Add a player:
```bash
near call player.$NEAR_ACCT add_player '{"hero_class": "Ogre", "name": "Roshan", "game_num": 1}' --accountId $NEAR_ACCT
near view player.$NEAR_ACCT get_game_players '{"game_num": 1}'
near call player.$NEAR_ACCT use_ability '{"ability": "strike", "game_num": 1}' --accountId $NEAR_ACCT
```

Add a new type of player:
```bash
near call player.$NEAR_ACCT add_player '{"hero_class": "Cleric", "name": "Warlock", "game_num": 1}' --accountId $NEAR_ACCT
near view player.$NEAR_ACCT get_game_players '{"game_num": 1}'
```