### An example lightweight event sourcing system for payments

Supports the following API operations:

- Deposit (account_number, amount)
- Withdraw (account_number, amount)
- Transfer (originator_account, beneficiary_account, amount) 

In this example prohect, the operations payload transform to events and are published to a system of record (in this example, nats streaming).

The µservice consumes the events from the system of record, calculates the balance for each account and updates a state database 

### Running

Install [Docker](https://docs.docker.com/engine/installation/) & [Docker-Compose](https://docs.docker.com/compose/install/)

Then:

```bash
git clone https://github.com/SharonU/test-app test-app
cd test-app
# start it up
docker-compose up
```

Open [http://localhost:3000](http://localhost:3000) (or `{docker ip}:3000` on windows) to view it in the browser.
