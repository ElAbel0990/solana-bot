##   What do we need to do?

the goal is create a bot to farm solana testnet and devnets tokens.

`solana config get`

`solana config set --url [Moniker]`

`solana airdrop --url localhost 1 AvGLCofJrZF8xrLn93M9ybuq18LFS53yHmUTauZo8bGV`
<br>**localhost did not work</br>

solana airdrop --url testnet 1 AvGLCofJrZF8xrLn93M9ybuq18LFS53yHmUTauZo8bGV



## TODO
- [x] Uderstand how to check provider connection
- [x] Calling the provieder to airdrop SOL
- [x] Now we need to call the command in rust
  - [x] We need to fuigure out how we can call 
  - [x] call echo
  - [x] try calling our airdrop command
- [ ] We want to make this a cron job
- [ ] first we want to call job unce evry 30secs
- [ ] we want to learn exit the job
- [ ] than we want to be able to set to 24.25hours
