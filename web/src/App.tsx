import * as React from 'react';
import './App.css';

import logo from './logo.svg';

import { Tab, TabList, TabPanel, Tabs } from 'react-tabs';
import "react-tabs/style/react-tabs.css";

class App extends React.Component<{}, {
  account: number,
  amount: number,
  beneficiaryAccount: number,
  originatorAccount: number,
  status: string
}> {
  constructor(props: object) {
    super(props);

    this.state = {
      account: 0,
      amount: 0,
      beneficiaryAccount: 0,
      originatorAccount: 0,
      status: ""
    };

    this.handleChange = this.handleChange.bind(this);
    this.handleDepositSubmit = this.handleDepositSubmit.bind(this);
    this.handleWithdrawSubmit = this.handleWithdrawSubmit.bind(this);
    this.handleTransferSubmit = this.handleTransferSubmit.bind(this);
  }

  public handleChange(event: React.ChangeEvent<HTMLInputElement>) {
    const target = event.target;
    const value = target.value;
    const name = target.name;

    let newState = {};

    // TODO: to avoid this, use two-way binding
    switch (name) {
      case "account":
        newState = { "account": parseInt(value, 10) };
        break;
      case "originatorAccount":
        newState = { "originatorAccount": parseInt(value, 10) };
        break;
      case "beneficiaryAccount":
        newState = { "beneficiaryAccount": parseInt(value, 10) };
        break;
      case "amount":
        newState = { "amount": parseInt(value, 10) };
        break;
    }

    if (newState !== {}) {
      this.setState(newState);
    }
  }

  public handleDepositSubmit(event: React.FormEvent<HTMLFormElement>) {
    this.postAction("/api/deposit", {
      account: this.state.account,
      amount: this.state.amount
    });

    event.preventDefault();
  }

  public handleWithdrawSubmit(event: React.FormEvent<HTMLFormElement>) {
    this.postAction("/api/withdraw", {
      account: this.state.account,
      amount: this.state.amount
    });

    event.preventDefault();
  }

  public handleTransferSubmit(event: React.FormEvent<HTMLFormElement>) {
    this.postAction("/api/transfer", {
      amount: this.state.amount,
      beneficiary_account: this.state.beneficiaryAccount,
      originator_account: this.state.originatorAccount
    });

    event.preventDefault();
  }

  // TODO: nitify using two-way binding

  public render() {
    return (
      <div className="App">
        <header className="App-header">
          <img src={logo} className="App-logo" alt="logo" />
          <h1 className="App-title">Test Application</h1>
        </header>
        <p className="App-intro">
          <Tabs>
            <TabList>
              <Tab>Deposit</Tab>
              <Tab>Withdraw</Tab>
              <Tab>Transfer</Tab>
            </TabList>

            <TabPanel>
              <h2>Deposit</h2>
              <div className="Form-Wrapper">
                <form onSubmit={this.handleDepositSubmit}>
                    <p>
                      <label>
                        Account
                        <input type="text" name="account" value={this.state.account} onChange={this.handleChange} />
                      </label>
                    </p>
                    <p>
                      <label>
                        Amount
                        <input type="text" name="amount" value={this.state.amount} onChange={this.handleChange} />
                      </label>
                    </p>
                    <p>
                      <button>Submit</button>
                    </p>
                </form>
              </div>
            </TabPanel>

            <TabPanel>
              <h2>Withdraw</h2>
              <div className="Form-Wrapper">
                <form onSubmit={this.handleWithdrawSubmit}>
                    <p>
                      <label>
                        Account
                        <input type="text" name="account" value={this.state.account} onChange={this.handleChange} />
                      </label>
                    </p>
                    <p>
                      <label>
                        Amount
                        <input type="text" name="amount" value={this.state.amount} onChange={this.handleChange} />
                      </label>
                    </p>
                    <p>
                      <button>Submit</button>
                    </p>
                </form>
              </div>
            </TabPanel>

            <TabPanel>
              <h2>Transfer</h2>
              <div className="Form-Wrapper">
                <form onSubmit={this.handleTransferSubmit}>
                    <p>
                      <label>
                        Originator&nbsp;Account
                        <input type="text" name="originatorAccount" value={this.state.originatorAccount} onChange={this.handleChange} />
                      </label>
                    </p>
                    <p>
                      <label>
                        Beneficiary&nbsp;Account
                        <input type="text" name="beneficiaryAccount" value={this.state.beneficiaryAccount} onChange={this.handleChange} />
                      </label>
                    </p>
                    <p>
                      <label>
                        Amount
                        <input type="text" name="amount" value={this.state.amount} onChange={this.handleChange} />
                      </label>
                    </p>
                    <p>
                      <button>Submit</button>
                    </p>
                </form>
              </div>
            </TabPanel>

          </Tabs>
        </p>
        <p className="Status">
          {this.state.status}
        </p>
      </div>
    );
  }

  private postAction(endpoint : string, json : object) {
    const args : RequestInit = {
        body: JSON.stringify(json),
        headers: {
          'Accept': 'application/json',
          'Content-Type': 'application/json'
        },
        method: "POST"
    }

    fetch(endpoint, args)
      .then(r => r.status !== 200
        ? `The server reported an error`
        : r.text()
      )
      .then(status =>
        this.setState({ "status": status })
      );
  }
}

export default App;
