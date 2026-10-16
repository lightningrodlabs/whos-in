import { assert, test } from "vitest";

import { runScenario, dhtSync } from '@holochain-open-dev/tryorama';
import { Record } from '@holochain/client';

import { sampleTwilioCredentials } from './common.js';

// Two-agent runtime check for the notifications zome's unrestricted cap grant
// (vendored notifications_coordinator, twilio_credentials.rs). That grant is not
// created in init: create_twilio_credentials creates it. Both agents call it, then
// Bob sends his contact to Alice with call_remote into her create_contact, which only
// succeeds if Alice's grant is valid. Alice then reads back what Bob sent.
test('notifications cap grant: Bob call_remotes create_contact on Alice', async () => {
  await runScenario(async scenario => {
    const testAppPath = process.cwd() + '/../workdir/whosin.happ';
    const appSource = { appBundleSource: { type: 'path' as const, value: testAppPath } };
    const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);
    await scenario.shareAllAgents();

    // Each agent creates Twilio credentials, which creates its cap grant.
    for (const player of [alice, bob]) {
      const record: Record = await player.cells[0].callZome({
        zome_name: "notifications",
        fn_name: "create_twilio_credentials",
        payload: await sampleTwilioCredentials(player.cells[0]),
      });
      assert.ok(record);
    }
    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Bob picks Alice as his notifier and sends her his contact.
    await bob.cells[0].callZome({
      zome_name: "notifications",
      fn_name: "select_notifier",
      payload: alice.agentPubKey,
    });
    const contact = {
      agent_pub_key: bob.agentPubKey,
      text_number: "+15550100",
      whatsapp_number: null,
      email_address: "bob@example.org",
    };
    await bob.cells[0].callZome({
      zome_name: "notifications",
      fn_name: "send_contact",
      payload: contact,
    });

    // Alice's chain now holds the contact Bob sent through her grant.
    const contacts: any[] = await alice.cells[0].callZome({
      zome_name: "notifications",
      fn_name: "get_contacts",
      payload: [bob.agentPubKey],
    });
    assert.equal(contacts.length, 1);
    assert.deepEqual(contacts[0], contact);
  });
});

// Control: without Alice's grant, the same call_remote is refused. This shows the
// test above depends on the grant and not on some other path.
test('notifications cap grant: call_remote is refused before Alice creates her grant', async () => {
  await runScenario(async scenario => {
    const testAppPath = process.cwd() + '/../workdir/whosin.happ';
    const appSource = { appBundleSource: { type: 'path' as const, value: testAppPath } };
    const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);
    await scenario.shareAllAgents();

    // Run init on Alice's cell without creating the grant.
    await alice.cells[0].callZome({
      zome_name: "notifications",
      fn_name: "get_contacts",
      payload: [],
    });

    await bob.cells[0].callZome({
      zome_name: "notifications",
      fn_name: "select_notifier",
      payload: alice.agentPubKey,
    });
    let error: any;
    try {
      await bob.cells[0].callZome({
        zome_name: "notifications",
        fn_name: "send_contact",
        payload: {
          agent_pub_key: bob.agentPubKey,
          text_number: "+15550100",
          whatsapp_number: null,
          email_address: null,
        },
      });
    } catch (e) {
      error = e;
    }
    assert.ok(error, "send_contact should fail without Alice's grant");
    assert.include(String(error?.message ?? error), "Failed to handle remote call");
  });
});
