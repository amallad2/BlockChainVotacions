## Deploy 

$ mxpy contract deploy --bytecode ./output/crowdfunding.wasm --proxy=https://devnet-gateway.multiversx.com --recall-nonce --arguments 10000000000000000000 1740753279 10 100 500 --gas-limit 20000000 --pem=wallet.pem --send

A sota he posat el deploy amb l'output 

## Crides al contracte

$ mxpy contract call erd1qqqqqqqqqqqqqpgqd8ras53afz6t3kxqk3khsmrqq4qsw2q3ll9szf5neg --pem=wallet.pem --recall-nonce --proxy=https://devnet-gateway.multiversx.com --chain D --function fund --value 10 --gas-limit 10000000 --send
INFO     utils: View this transaction in the MultiversX Devnet Explorer:                        utils.py:218
         https://devnet-explorer.multiversx.com/transactions/1eb1486e1107612a453d29affd075f8f6a             
         7407fbe62963714e0d89a13824eec2                                                                     
{
    "emittedTransaction": {
        "nonce": 4,
        "value": "10",
        "receiver": "erd1qqqqqqqqqqqqqpgqd8ras53afz6t3kxqk3khsmrqq4qsw2q3ll9szf5neg",
        "sender": "erd1lzrpyn0krqtcss46murjtpgr22d70av5c3xa8esua4znvek6ll9s4nhrg8",
        "senderUsername": "",
        "receiverUsername": "",
        "gasPrice": 1000000000,
        "gasLimit": 10000000,
        "data": "ZnVuZA==",
        "chainID": "D",
        "version": 2,
        "options": 0,
        "guardian": "",
        "signature": "8462d6c9db04d2531d4ac85d86220fd745f0f4fee40e2de8345f011a314d2e485d20e09cfef26e129cb2e75273a65d2d70e0b3b2868795c2527ad8c93d7a3c0b",
        "guardianSignature": "",
        "relayer": "",
        "relayerSignature": ""
    },
    "emittedTransactionData": "fund",
    "emittedTransactionHash": "1eb1486e1107612a453d29affd075f8f6a7407fbe62963714e0d89a13824eec2",
    "contractAddress": "erd1qqqqqqqqqqqqqpgqd8ras53afz6t3kxqk3khsmrqq4qsw2q3ll9szf5neg"
}


~/blockchain$ mxpy contract call erd1qqqqqqqqqqqqqpgqd8ras53afz6t3kxqk3khsmrqq4qsw2q3ll9szf5neg --pem=wallet.pem --recall-nonce --proxy=https://devnet-gateway
.multiversx.com --chain D --function fund --value 800 --gas-limit 10000000 --send
INFO     utils: View this transaction in the MultiversX Devnet Explorer:                                                                                          utils.py:218
         https://devnet-explorer.multiversx.com/transactions/d68baeaf870e01ff7f8e08eec4bd5fc927933dfe19a17e6ccc353f134be28b2b                                                 
{
    "emittedTransaction": {
        "nonce": 5,
        "value": "800",
        "receiver": "erd1qqqqqqqqqqqqqpgqd8ras53afz6t3kxqk3khsmrqq4qsw2q3ll9szf5neg",
        "sender": "erd1lzrpyn0krqtcss46murjtpgr22d70av5c3xa8esua4znvek6ll9s4nhrg8",
        "senderUsername": "",
        "receiverUsername": "",
        "gasPrice": 1000000000,
        "gasLimit": 10000000,
        "data": "ZnVuZA==",
        "chainID": "D",
        "version": 2,
        "options": 0,
        "guardian": "",
        "signature": "580c463e6bf41970b6fb5d649233e3c4aa0f0df6209f5c94066050aed9a2e7afa600604b443fbb5a07fd4cee2975f4589ccec2a939bc3a77a75e3152a6653e0d",
        "guardianSignature": "",
        "relayer": "",
        "relayerSignature": ""
    },
    "emittedTransactionData": "fund",
    "emittedTransactionHash": "d68baeaf870e01ff7f8e08eec4bd5fc927933dfe19a17e6ccc353f134be28b2b",
    "contractAddress": "erd1qqqqqqqqqqqqqpgqd8ras53afz6t3kxqk3khsmrqq4qsw2q3ll9szf5neg"
}

$ mxpy contract call erd1qqqqqqqqqqqqqpgq4rlulpc7smh8fsr6l9tn33almrraszxall9spg70kw --pem=wallet.pem --recall-nonce --proxy=https://devnet-gateway.multiversx.com --chain D --function fund --value 10 --gas-limit 10000000 --send

Transaction details

https://devnet-explorer.multiversx.com/transactions/533ec1458a80dc8a1a70dfb66169be51105a18ba8f2fc7c1ee8cfd64b680e759


## Deploy Contracte 

Amb Valors:
- min_amount = 10
- max_amount_per_donor = 100
- max_amount_total = 500

$ mxpy contract deploy --bytecode ./output/crowdfunding.wasm --proxy=https://devnet-gateway.multiversx.com --recall-nonce --arguments 10000000000000000000 1740753279 10 100 500 --gas-limit 20000000 --pem=wallet.pem --send
INFO     cli.contracts: Contract address:                                               cli_contracts.py:349
         erd1qqqqqqqqqqqqqpgqd8ras53afz6t3kxqk3khsmrqq4qsw2q3ll9szf5neg                                     
INFO     utils: View this contract address in the MultiversX Devnet Explorer:                   utils.py:218
         https://devnet-explorer.multiversx.com/accounts/erd1qqqqqqqqqqqqqpgqd8ras53afz6t3kxqk3             
         khsmrqq4qsw2q3ll9szf5neg                                                                           
INFO     utils: View this transaction in the MultiversX Devnet Explorer:                        utils.py:218
         https://devnet-explorer.multiversx.com/transactions/5703d76e53992401d4cf2801276071ac17             
         4d345578b2816e63c87358f3ef1951                                                                     
{
    "emittedTransaction": {
        "nonce": 3,
        "value": "0",
        "receiver": "erd1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq6gq4hu",
        "sender": "erd1lzrpyn0krqtcss46murjtpgr22d70av5c3xa8esua4znvek6ll9s4nhrg8",
        "senderUsername": "",
        "receiverUsername": "",
        "gasPrice": 1000000000,
        "gasLimit": 20000000,
        "data": "MDA2MTczNmQwMTAwMDAwMDAxNTQwZjYwMDI3ZjdmMDE3ZjYwMDAwMDYwMDE3ZjAwNjAwMDAxN2Y2MDAxN2YwMTdmNjAwMjdmN2YwMDYwMDM3ZjdmN2YwMTdmNjAwMTdmMDE3ZTYwMDU3ZjdmN2Y3ZTdmMDA2MDAwMDE3ZTYwMDU3ZjdmN2U3ZjdmMDE3ZjYwMDI3ZjdlMDA2MDA0N2Y3ZjdmN2YwMTdmNjAwMzdmN2Y3ZjAwNjAwMTdlMDAwMmI4MDYyMTAzNjU2ZTc2MGI3MzY5Njc2ZTYxNmM0NTcyNzI2ZjcyMDAwNTAzNjU2ZTc2MTA2ZDQyNzU2NjY2NjU3MjQ3NjU3NDRjNjU2ZTY3NzQ2ODAwMDQwMzY1NmU3NjBmNmQ0Mjc1NjY2NjY1NzI0NzY1NzQ0Mjc5NzQ2NTczMDAwMDAzNjU2ZTc2MWM2MjY5Njc0OTZlNzQ0NzY1NzQ0NTUzNDQ1NDQ1Nzg3NDY1NzI2ZTYxNmM0MjYxNmM2MTZlNjM2NTAwMDgwMzY1NmU3NjE4NjI2OTY3NDk2ZTc0NDc2NTc0NDU3ODc0NjU3MjZlNjE2YzQyNjE2YzYxNmU2MzY1MDAwNTAzNjU2ZTc2MTE2NzY1NzQ0MjZjNmY2MzZiNTQ2OTZkNjU3Mzc0NjE2ZDcwMDAwOTAzNjU2ZTc2MWI2ZDYxNmU2MTY3NjU2NDU0NzI2MTZlNzM2NjY1NzI1NjYxNmM3NTY1NDU3ODY1NjM3NTc0NjUwMDBhMDM2NTZlNzYwZDZkNjE2ZTYxNjc2NTY0NDM2MTZjNmM2NTcyMDAwMjAzNjU2ZTc2MTA2ZDYxNmU2MTY3NjU2NDUzNDM0MTY0NjQ3MjY1NzM3MzAwMDIwMzY1NmU3NjEyNjI2OTY3NDk2ZTc0NDc2NTc0NDM2MTZjNmM1NjYxNmM3NTY1MDAwMjAzNjU2ZTc2MTI2ZDQyNzU2NjY2NjU3MjQxNzA3MDY1NmU2NDQyNzk3NDY1NzMwMDA2MDM2NTZlNzYxMjZkNjE2ZTYxNjc2NTY0NTM2OTY3NmU2MTZjNDU3MjcyNmY3MjAwMDIwMzY1NmU3NjE5NjI2OTY3NDk2ZTc0NDc2NTc0NTU2ZTczNjk2NzZlNjU2NDQxNzI2Nzc1NmQ2NTZlNzQwMDA1MDM2NTZlNzYxYjczNmQ2MTZjNmM0OTZlNzQ0NzY1NzQ1NTZlNzM2OTY3NmU2NTY0NDE3MjY3NzU2ZDY1NmU3NDAwMDcwMzY1NmU3NjEyNmQ0Mjc1NjY2NjY1NzI0NzY1NzQ0MTcyNjc3NTZkNjU2ZTc0MDAwMDAzNjU2ZTc2MGY2NzY1NzQ0ZTc1NmQ0MTcyNjc3NTZkNjU2ZTc0NzMwMDAzMDM2NTZlNzYxNDYyNjk2NzQ5NmU3NDQ2Njk2ZTY5NzM2ODU1NmU3MzY5Njc2ZTY1NjQwMDAyMDM2NTZlNzYwYTYyNjk2NzQ5NmU3NDUzNjk2NzZlMDAwNDAzNjU2ZTc2MGU2MjY5Njc0OTZlNzQ1MzY1NzQ0OTZlNzQzNjM0MDAwYjAzNjU2ZTc2MTM2ZDQyNzU2NjY2NjU3MjQ3NjU3NDQyNzk3NDY1NTM2YzY5NjM2NTAwMGMwMzY1NmU3NjBmNmQ0Mjc1NjY2NjY1NzI1MzY1NzQ0Mjc5NzQ2NTczMDAwNjAzNjU2ZTc2MTI2ZDQyNzU2NjY2NjU3MjUzNzQ2ZjcyNjE2NzY1NGM2ZjYxNjQwMDAwMDM2NTZlNzYxOTZkNDI3NTY2NjY2NTcyNDY3MjZmNmQ0MjY5Njc0OTZlNzQ1NTZlNzM2OTY3NmU2NTY0MDAwMDAzNjU2ZTc2MTM2ZDQyNzU2NjY2NjU3MjUzNzQ2ZjcyNjE2NzY1NTM3NDZmNzI2NTAwMDAwMzY1NmU3NjE3NmQ0Mjc1NjY2NjY1NzI1NDZmNDI2OTY3NDk2ZTc0NTU2ZTczNjk2NzZlNjU2NDAwMDAwMzY1NmU3NjBkNmQ0Mjc1NjY2NjY1NzI0MTcwNzA2NTZlNjQwMDAwMDM2NTZlNzYwOTYyNjk2NzQ5NmU3NDQzNmQ3MDAwMDAwMzY1NmU3NjBlNjM2ODY1NjM2YjRlNmY1MDYxNzk2ZDY1NmU3NDAwMDEwMzY1NmU3NjFjNmQ2MTZlNjE2NzY1NjQ0NzY1NzQ0ZDc1NmM3NDY5NDU1MzQ0NTQ0MzYxNmM2YzU2NjE2Yzc1NjUwMDAyMDM2NTZlNzYwOTYyNjk2NzQ5NmU3NDQxNjQ2NDAwMGQwMzY1NmU3NjEzNmQ2MTZlNjE2NzY1NjQ0Zjc3NmU2NTcyNDE2NDY0NzI2NTczNzMwMDAyMDM2NTZlNzYxNjczNmQ2MTZjNmM0OTZlNzQ0NjY5NmU2OTczNjg1NTZlNzM2OTY3NmU2NTY0MDAwZTAzNjU2ZTc2MDk2ZDQyNzU2NjY2NjU3MjQ1NzEwMDAwMDMyMzIyMDMwMDA0MDMwMjAzMDMwNzAzMDQwNTA1MDMwMzAzMDQwMjAyMDQwNjAwMDQwNTA0MDEwMTAxMDEwMTAxMDEwMTAxMDEwNTAzMDEwMDAzMDYxNjAzN2YwMTQxODA4MDA4MGI3ZjAwNDFjOTgzMDgwYjdmMDA0MWQwODMwODBiMDc5MjAxMGQwNjZkNjU2ZDZmNzI3OTAyMDAwNDY5NmU2OTc0MDAzOTA3NzU3MDY3NzI2MTY0NjUwMDNhMDQ2Njc1NmU2NDAwM2IwNTYzNmM2MTY5NmQwMDNjMDY3Mzc0NjE3NDc1NzMwMDNkMGY2NzY1NzQ0Mzc1NzI3MjY1NmU3NDQ2NzU2ZTY0NzMwMDNlMDk2NzY1NzQ1NDYxNzI2NzY1NzQwMDNmMGI2NzY1NzQ0NDY1NjE2NDZjNjk2ZTY1MDA0MDBhNjc2NTc0NDQ2NTcwNmY3MzY5NzQwMDQxMDg2MzYxNmM2YzQyNjE2MzZiMDA0MjBhNWY1ZjY0NjE3NDYxNWY2NTZlNjQwMzAxMGI1ZjVmNjg2NTYxNzA1ZjYyNjE3MzY1MDMwMjBhZjEwYjIyNTYwMTA0N2Y0MWM3ODIwODQxMGIxMDIyMjIwMjEwMjMxMDI0MjIwMDEwMDgyMDAwMjEwMzEwMjQyMTAwNDUwNDQwMjAwMjEwMDEyMTAxMjAwMzEwMjUyMDAyNDFhOTgzMDgxMDAyMWE0MTg5ODMwODQxYTk4MzA4MjAwMTQyMDAyMDAwMTAwMzIwMDAwZjBiMjAwMzEwMjU0MTg5ODMwODIwMDAxMDA0MjAwMDBiMTEwMTAxN2YxMDI0MjIwMjIwMDAyMDAxMTAxNDFhMjAwMjBiMTMwMDQxNTg0MWM3ODIwODQxMGIxMDE0MWE0MTU4MjAwMDEwMzUwYjE5MDEwMTdmNDE4NDgzMDg0MTg0ODMwODI4MDIwMDQxMDE2YjIyMDAzNjAyMDAyMDAwMGIwYjAwMjAwMDQxODk4MzA4MTAwMjFhMGIzMjAxMDE3ZjEwMDUxMDI3MTAyODU2MDQ3ZjQxMDE0MTAyNDE3ZjEwMjExMDI5MTAyYTEwMWEyMjAwNDEwMDQ3MjAwMDQxMDA0ODFiNDFmZjAxNzE0MTAyNDkxYjA1NDEwMDBiMGIwYTAwNDE5OTgxMDg0MTA4MTAyMjBiYzAwMTAyMDM3ZjAxN2UyMzAwNDExMDZiMjIwMTI0MDAyMDAxNDIwMDM3MDMwODIwMDAxMDM2MjIwMzEwMDEyMjAyNDEwOTRmMDQ0MDQxZDI4MjA4NDExYjEwMjIyMjAxMjAwMDEwMTkxYTIwMDE0MTgwODIwODQxMDMxMDBhMWEyMDAxNDE5YzgyMDg0MTBlMTAwYTFhMjAwMTEwMGIwMDBiMjAwMzIwMDEyMDAyNmI0MTEwNmEyMDAyMTAzNDFhMjAwMTI5MDMwODIxMDQyMDAxNDExMDZhMjQwMDIwMDQ0MjM4ODYyMDA0NDI4MGZlMDM4MzQyMjg4Njg0MjAwNDQyODA4MGZjMDc4MzQyMTg4NjIwMDQ0MjgwODA4MGY4MGY4MzQyMDg4Njg0ODQyMDA0NDIwODg4NDI4MDgwODBmODBmODMyMDA0NDIxODg4NDI4MDgwZmMwNzgzODQyMDA0NDIyODg4NDI4MGZlMDM4MzIwMDQ0MjM4ODg4NDg0ODQwYjBhMDA0MWExODEwODQxMDYxMDIyMGIwZjAwMjAwMDEwMzYxMDI0MjIwMDEwMTgxYTIwMDAwYjA5MDAyMDAwMjAwMTEwMDAwMDBiMGYwMDIwMDAyMDAxNDIwMDEwMmQxMDJkMTAwNjFhMGIwODAwNDEwMTQxMDAxMDIyMGIwYzAxMDE3ZjEwMjQyMjAwMTAwNzIwMDAwYjEwMDA0MTAxMTAzMDQ1MDQ0MDQxNWQxMDA5MGI0MTVkMGIyYjAxMDI3ZjIwMDA0MTg4ODMwODJkMDAwMDIyMDE3MTIwMDA0MWZmMDE3MTQ2MjIwMjQ1MDQ0MDQxODg4MzA4MjAwMDIwMDE3MjNhMDAwMDBiMjAwMjBiMTQwMDEwMGYyMDAwNDYwNDQwMGYwYjQxODM4MjA4NDExOTEwMDAwMDBiMDgwMDIwMDAxMDJhMTAxMDBiMTMwMDQxN2YyMDAwMTAxMTIyMDA0MTAwNDcyMDAwNDEwMDQ4MWIwYjBmMDAyMDAwNDEwMDIwMDIyMDAxMTAxMzQxMDA0NzBiMGIwMDIwMDAyMDAxMTAyMDQxMDA0YTBiMGQwMDIwMDAxMDI0MjIwMDEwMTUxYTIwMDAwYjE0MDEwMTdmMTAyNDIyMDIyMDAxMTAxNjFhMjAwMDIwMDIxMDE3MWEwYjE1MDEwMTdmNDFmZDgyMDg0MTA3MTAyMjIyMDEyMDAwMTAxOTFhMjAwMTBiYmIwMjAyMDM3ZjAyN2UyMzAwNDExMDZiMjIwMjI0MDAxMDFiNDEwMjEwMzE0MTAwMTAyNDIyMDAxMDBjNDEwMTEwMGQyMTAzMDI0MDIwMDAxMDMzNDFmZjAxNzE0MTAxNDYwNDQwMTAyOTIwMDAxMDM3MTAwNTIwMDM1YTBkMDExMDI3MjAwMjIwMDM0MjM4ODYyMDAzNDI4MGZlMDM4MzQyMjg4Njg0MjAwMzQyODA4MGZjMDc4MzQyMTg4NjIwMDM0MjgwODA4MGY4MGY4MzQyMDg4Njg0ODQyMDAzNDIwODg4NDI4MDgwODBmODBmODMyMDAzNDIxODg4NDI4MDgwZmMwNzgzODQyMDAzNDIyODg4MjIwNDQyODBmZTAzODMyMDAzNDIzODg4ODQ4NDg0MzcwMzA4NDEwMDIwMDM0MjgwODA4MDgwODA4MDgwODAwMTU0MjIwMDIwMDM0MjMwODhhNzQxZmYwMTcxMWIyMjAxMjAwMDZhNDEwMDIwMDEyMDA0YTc0MWZmMDE3MTFiMjIwMDZhNDEwMDIwMDAyMDAzNDIyMDg4YTc0MWZmMDE3MTFiMjIwMDZhNDEwMDIwMDAyMDAzYTcyMjAwNDExODc2MWIyMjAxNmE0MTAwMjAwMTIwMDA0MTEwNzY0MWZmMDE3MTFiMjIwMTZhNDEwMDIwMDEyMDAwNDEwODc2NDFmZjAxNzExYjZhMjIwMDIwMDI0MTA4NmE2YTQxMDgyMDAwNmIxMDIyMTAxNzFhMjAwMjQxMTA2YTI0MDAwZjBiNDE5YTgwMDg0MTFhMTAyYjAwMGI0MWI0ODAwODQxMWQxMDJiMDAwYjA4MDAxMDFiNDEwMDEwMzEwYmU2MDEwMTA0N2YyMzAwNDExMDZiMjIwMTI0MDA0MTAyMTAzMDQ1MDQ0MDQxNWExMDFjMGIwMjQwMDI0MDAyNDAwMjQwMDI0MDAyNDA0MTVhMTAwMTQxMDQ3NjBlMDIwMjAxMDAwYjQxY2M4MTA4NDExZDEwMDAwMDBiMjAwMTQxMDg2YTQyMDAzNzAzMDAyMDAxNDIwMDM3MDMwMDQxNWEyMDAxNDExMDEwMzQwZDAyMjAwMTI4MDIwMDIyMDA0MTE4NzQyMDAwNDE4MGZlMDM3MTQxMDg3NDcyMjAwMDQxMDg3NjQxODBmZTAzNzEyMDAwNDExODc2NzI3MjEwMjMwZDAxNDFhNzgxMDg0MTI1MTAwMDAwMGIxMDJmMWEwYjQxMDAxMDMxMTAyZjIxMDIxMDI0MjIwMDQyMDAxMDEyMjAwMDIwMDAyMDAyMTAxZDEwMDUxMDI3MTAyODVhMGQwMTEwMmUyMjAzMTAzODEwMmEyMTAyMjAwMzEwMzgyMDAyMjAwMjIwMDAxMDFkMjAwMjEwMzcyMDAxNDExMDZhMjQwMDBmMGI0MWFhODIwODQxMWQxMDAwMDAwYjQxODA4MDA4NDExYTEwMmIwMDBiN2YwMTAzN2YxMDFiNDEwMDEwMzEwMjQwMDI0MDAyNDAwMjQwMTAyNjQxZmYwMTcxNDEwMTZiMGUwMjAxMDIwMDBiNDFkMTgwMDg0MTFjMTAyYjAwMGIxMDJlMjEwMDEwMjQyMjAxMTAxZTIwMDAyMDAxMTAzNTQ1MGQwMTIwMDAxMDIxMTAyYzBmMGIxMDJlMjIwMDEwMzgxMDJhMjIwMTEwMzM0MWZmMDE3MTQxMDE0NjA0NDAyMDAwMTAzODQxNmM0MTAxNDEwMDEwMTQxYTQxNmMxMDE3MWEyMDAwMjAwMTEwMmMwYjBmMGI0MWVkODAwODQxMjcxMDJiMDAwYjExMDAxMDFiNDEwMDEwMzExMDI2NDFmZjAxNzFhZDEwMWYwYjBjMDAxMDFiNDEwMDEwMzExMDIxMTAxMDBiMGMwMDEwMWI0MTAwMTAzMTEwMjkxMDMyMGIwZTAwMTAxYjQxMDAxMDMxMTAyNzEwMjgxMDFmMGI1MTAxMDE3ZjEwMWI0MTAxMTAzMTQxMDAxMDI0MjIwMDEwMGUxYTIwMDAxMDAxNDEyMDQ3MDQ0MDQxZTk4MTA4NDExNzEwMjIyMjAwNDE5NDgxMDg0MTA1MTAwYTFhMjAwMDQxODA4MjA4NDEwMzEwMGExYTIwMDA0MWVkODIwODQxMTAxMDBhMWEyMDAwMTAwYjAwMGIyMDAwMTAzODEwMzIwYjAyMDAwYjBiOTgwMzAyMDA0MTgwODAwODBiODQwMzYzNjE2ZTZlNmY3NDIwNjY3NTZlNjQyMDYxNjY3NDY1NzIyMDY0NjU2MTY0NmM2OTZlNjU1NDYxNzI2NzY1NzQyMDZkNzU3Mzc0MjA2MjY1MjA2ZDZmNzI2NTIwNzQ2ODYxNmUyMDMwNDQ2NTYxNjQ2YzY5NmU2NTIwNjM2MTZlMjc3NDIwNjI2NTIwNjk2ZTIwNzQ2ODY1MjA3MDYxNzM3NDYzNjE2ZTZlNmY3NDIwNjM2YzYxNjk2ZDIwNjI2NTY2NmY3MjY1MjA2NDY1NjE2NDZjNjk2ZTY1NmY2ZTZjNzkyMDZmNzc2ZTY1NzIyMDYzNjE2ZTIwNjM2YzYxNjk2ZDIwNzM3NTYzNjM2NTczNzM2Njc1NmMyMDY2NzU2ZTY0Njk2ZTY3NjQ2ZjZlNmY3MjY0NjU2MTY0NmM2OTZlNjU3NDYxNzI2NzY1NzQ2Njc1NmU2Mzc0Njk2ZjZlMjA2NDZmNjU3MzIwNmU2Zjc0MjA2MTYzNjM2NTcwNzQyMDQ1NTM0NDU0MjA3MDYxNzk2ZDY1NmU3NDY5NmU2MzZmNzI3MjY1NjM3NDIwNmU3NTZkNjI2NTcyMjA2ZjY2MjA3NDcyNjE2ZTczNjY2NTcyNzM2MTcyNjc3NTZkNjU2ZTc0MjA2NDY1NjM2ZjY0NjUyMDY1NzI3MjZmNzIyMDI4MjkzYTIwNzc3MjZmNmU2NzIwNmU3NTZkNjI2NTcyMjA2ZjY2MjA2MTcyNjc3NTZkNjU2ZTc0NzM2OTZlNzA3NTc0MjA3NDZmNmYyMDZjNmY2ZTY3NGQ2MTZlNjE2NzY1NjQ1NjY1NjMyMDY5NmU2NDY1NzgyMDZmNzU3NDIwNmY2NjIwNzI2MTZlNjc2NTQ1NDc0YzQ0MmQzMDMwMzAzMDMwMzA3Mzc0NmY3MjYxNjc2NTIwNjQ2NTYzNmY2NDY1MjA2NTcyNzI2ZjcyMjAyODZiNjU3OTNhMjA2MjYxNjQyMDYxNzI3MjYxNzkyMDZjNjU2ZTY3NzQ2ODY0NjU3MDZmNzM2OTc0MDA0MTg0ODMwODBiMDQzOGZmZmZmZkAwNTAwQDA1MDBAOGFjNzIzMDQ4OWU4MDAwMEA2N2MxYzk3ZkAwYUA2NEAwMWY0",
        "chainID": "D",
        "version": 2,
        "options": 0,
        "guardian": "",
        "signature": "c2012028cf77d2f12f3532c6ef71ef2594aeb5a758dfbf2e069e4118dd9f232a8a1fbad14976768d6128caa9b0d76f24b6fdc1da83f88fc5c87671fd5ebbca09",
        "guardianSignature": "",
        "relayer": "",
        "relayerSignature": ""
    },
    "emittedTransactionData": "0061736d0100000001540f60027f7f017f60000060017f006000017f60017f017f60027f7f0060037f7f7f017f60017f017e60057f7f7f7e7f006000017e60057f7f7e7f7f017f60027f7e0060047f7f7f7f017f60037f7f7f0060017e0002b8062103656e760b7369676e616c4572726f72000503656e76106d4275666665724765744c656e677468000403656e760f6d4275666665724765744279746573000003656e761c626967496e744765744553445445787465726e616c42616c616e6365000803656e7618626967496e7447657445787465726e616c42616c616e6365000503656e7611676574426c6f636b54696d657374616d70000903656e761b6d616e616765645472616e7366657256616c756545786563757465000a03656e760d6d616e6167656443616c6c6572000203656e76106d616e61676564534341646472657373000203656e7612626967496e7447657443616c6c56616c7565000203656e76126d427566666572417070656e644279746573000603656e76126d616e616765645369676e616c4572726f72000203656e7619626967496e74476574556e7369676e6564417267756d656e74000503656e761b736d616c6c496e74476574556e7369676e6564417267756d656e74000703656e76126d427566666572476574417267756d656e74000003656e760f6765744e756d417267756d656e7473000303656e7614626967496e7446696e697368556e7369676e6564000203656e760a626967496e745369676e000403656e760e626967496e74536574496e743634000b03656e76136d42756666657247657442797465536c696365000c03656e760f6d4275666665725365744279746573000603656e76126d42756666657253746f726167654c6f6164000003656e76196d42756666657246726f6d426967496e74556e7369676e6564000003656e76136d42756666657253746f7261676553746f7265000003656e76176d427566666572546f426967496e74556e7369676e6564000003656e760d6d427566666572417070656e64000003656e7609626967496e74436d70000003656e760e636865636b4e6f5061796d656e74000103656e761c6d616e616765644765744d756c74694553445443616c6c56616c7565000203656e7609626967496e74416464000d03656e76136d616e616765644f776e657241646472657373000203656e7616736d616c6c496e7446696e697368556e7369676e6564000e03656e76096d427566666572457100000323220300040302030307030405050303030402020406000405040101010101010101010105030100030616037f01418080080b7f0041c983080b7f0041d083080b0792010d066d656d6f7279020004696e697400390775706772616465003a0466756e64003b05636c61696d003c06737461747573003d0f67657443757272656e7446756e6473003e09676574546172676574003f0b676574446561646c696e6500400a6765744465706f73697400410863616c6c4261636b00420a5f5f646174615f656e6403010b5f5f686561705f6261736503020af10b225601047f41c78208410b102222021023102422001008200021031024210045044020021001210120031025200241a9830810021a4189830841a98308200142002000100320000f0b20031025418983082000100420000b1101017f102422022000200110141a20020b1300415841c78208410b10141a4158200010350b1901017f418483084184830828020041016b220036020020000b0b0020004189830810021a0b3201017f10051027102856047f41014102417f10211029102a101a220041004720004100481b41ff01714102491b0541000b0b0a0041998108410810220bc00102037f017e230041106b22012400200142003703082000103622031001220241094f044041d28208411b10222201200010191a2001418082084103100a1a2001419c8208410e100a1a2001100b000b2003200120026b41106a200210341a20012903082104200141106a2400200442388620044280fe0383422886842004428080fc0783421886200442808080f80f834208868484200442088842808080f80f832004421888428080fc07838420044228884280fe038320044238888484840b0a0041a18108410610220b0f00200010361024220010181a20000b0900200020011000000b0f00200020014200102d102d10061a0b08004101410010220b0c01017f10242200100720000b100041011030450440415d10090b415d0b2b01027f2000418883082d0000220171200041ff01714622024504404188830820002001723a00000b20020b1400100f20004604400f0b4183820841191000000b08002000102a10100b1300417f20001011220041004720004100481b0b0f00200041002002200110134100470b0b0020002001102041004a0b0d0020001024220010151a20000b1401017f10242202200110161a2000200210171a0b1501017f41fd8208410710222201200010191a20010bbb0202037f027e230041106b22022400101b41021031410010242200100c4101100d210302402000103341ff01714101460440102920001037100520035a0d0110272002200342388620034280fe0383422886842003428080fc0783421886200342808080f80f834208868484200342088842808080f80f832003421888428080fc078384200342288822044280fe0383200342388884848437030841002003428080808080808080015422002003423088a741ff01711b220120006a410020012004a741ff01711b22006a410020002003422088a741ff01711b22006a410020002003a722004118761b22016a41002001200041107641ff01711b22016a41002001200041087641ff01711b6a2200200241086a6a410820006b102210171a200241106a24000f0b419a8008411a102b000b41b48008411d102b000b0800101b410010310be60101047f230041106b2201240041021030450440415a101c0b024002400240024002400240415a10014104760e020201000b41cc8108411d1000000b200141086a420037030020014200370300415a2001411010340d022001280200220041187420004180fe03714108747220004108764180fe03712000411876727210230d0141a7810841251000000b102f1a0b41001031102f21021024220042001012200020002002101d1005102710285a0d01102e22031038102a210220031038200220022000101d20021037200141106a24000f0b41aa8208411d1000000b41808008411a102b000b7f01037f101b410010310240024002400240102641ff017141016b0e020102000b41d18008411c102b000b102e210010242201101e200020011035450d0120001021102c0f0b102e22001038102a2201103341ff0171410146044020001038416c4101410010141a416c10171a20002001102c0b0f0b41ed80084127102b000b1100101b41001031102641ff0171ad101f0b0c00101b41001031102110100b0c00101b41001031102910320b0e00101b4100103110271028101f0b5101017f101b41011031410010242200100e1a20001001412047044041e98108411710222200419481084105100a1a2000418082084103100a1a200041ed82084110100a1a2000100b000b2000103810320b02000b0b98030200418080080b840363616e6e6f742066756e6420616674657220646561646c696e65546172676574206d757374206265206d6f7265207468616e2030446561646c696e652063616e277420626520696e20746865207061737463616e6e6f7420636c61696d206265666f726520646561646c696e656f6e6c79206f776e65722063616e20636c61696d207375636365737366756c2066756e64696e67646f6e6f72646561646c696e6574617267657466756e6374696f6e20646f6573206e6f74206163636570742045534454207061796d656e74696e636f7272656374206e756d626572206f66207472616e7366657273617267756d656e74206465636f6465206572726f722028293a2077726f6e67206e756d626572206f6620617267756d656e7473696e70757420746f6f206c6f6e674d616e6167656456656320696e646578206f7574206f662072616e676545474c442d30303030303073746f72616765206465636f6465206572726f7220286b65793a20626164206172726179206c656e6774686465706f73697400418483080b0438ffffff@0500@0500@8ac7230489e80000@67c1c97f@0a@64@01f4",
    "emittedTransactionHash": "5703d76e53992401d4cf2801276071ac174d345578b2816e63c87358f3ef1951",
    "contractAddress": "erd1qqqqqqqqqqqqqpgqd8ras53afz6t3kxqk3khsmrqq4qsw2q3ll9szf5neg"
}
