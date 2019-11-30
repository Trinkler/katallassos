// This is an example on how a contract deployment could look like.
// Use https://polkadot.js.org/apps/#/js and copy paste the below text.
// Note you might want to adjust Alice's address and use a non-trivial contract_id
// to successfully deploy this loan contract.
const ALICE = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY'
// Create a extrinsic, transferring randomAmount units to Bob.
const transfer = api.tx.contracts
  .dispatchDeploy({
    // here only add the values you want, the rest will be default, i.e.
    // all options to none, all vecs to 0 length
    contract_deal_date: {
      year: 2019,
      month: 1,
      day: 1,
      hour: 0,
      minute: 0,
      second: 0
    },
    contract_id: "0x0000000000000000000000000000000000000000000000000000000000000001",
    contract_role: 0, // ContractRole::RPA
    contract_type: 0, // ContractType::PAM
    counterparty_id: "0x0000000000000000000000000000000000000000000000000000000000000002",
    creator_id: "0x0000000000000000000000000000000000000000000000000000000000000003",
    currency: 1,
    day_count_convention: 2, // DayCountConvention::A365
    initial_exchange_date: {
      year: 2020,
      month: 1,
      day: 2,
      hour: 1,
      minute: 0,
      second: 0
    },
    maturity_date: {
      year: 2020,
      month: 4,
      day: 2,
      hour: 0,
      minute: 0,
      second: 0
    },
    nominal_interest_rate: "0x00000000000003e8",
    notional_principal: "0x0000000002faf080"
  });

// Sign and Send the transaction
transfer.signAndSend(ALICE, ({
  events = [],
  status
}) => {
  if (status.isFinalized) {
    console.log('Successful deployment of ' + ' with hash ' + status.asFinalized.toHex());
  } else {
    console.log('Status of deployment: ' + status.type);
  }

  events.forEach(({
    phase,
    event: {
      data,
      method,
      section
    }
  }) => {
    console.log(phase.toString() + ' : ' + section + '.' + method + ' ' + data.toString());
  });
});