const path = require('path')
const tape = require('tape')

const { Diorama, tapeExecutor, backwardCompatibilityMiddleware } = require('@holochain/diorama')

process.on('unhandledRejection', error => {
  // Will print "unhandledRejection err is not defined"
  console.error('got unhandledRejection:', error);
});

const dnaPath = path.join(__dirname, "../dist/holopass.dna.json")
const dna = Diorama.dna(dnaPath, 'holopass')

const diorama = new Diorama({
  instances: {
    alice: dna,
    bob: dna,
  },
  bridges: [],
  debugLog: false,
  executor: tapeExecutor(require('tape')),
  middleware: backwardCompatibilityMiddleware,
})

diorama.registerScenario("create new password", async (s, t, { alice }) => {
  // enter pass info from UI
  const addr = await alice.call("passwords", "store_password", {"domainname":"google.com", "username":"viktor", "password":"pass123"})
  console.log(addr)
  t.ok(addr)

})

diorama.registerScenario("get credentials for domain", async (s, t, { alice }) => {
  // enter pass info from UI
  const credential = await alice.call("passwords", "get_credentials_for_domain", {"domainname":"google.com"})
  console.log(credential)
  t.ok(credential)

})
/*
diorama.registerScenario("description of example test", async (s, t, { alice }) => {
  // Make a call to a Zome function
  // indicating the function, and passing it an input
  const addr = await alice.call("passwords", "create_my_entry", {"entry" : {"content":"sample content"}})
  const result = await alice.call("passwords", "get_my_entry", {"address": addr.Ok})

  // check for equality of the actual and expected results
  t.deepEqual(result, { Ok: { App: [ 'my_entry', '{"content":"sample content"}' ] } })
})
*/
diorama.run()
