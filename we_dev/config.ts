import { defineConfig } from '@lightningrodlabs/we-dev-cli';

export default defineConfig({
  groups: [
    {
      name: 'Lightning Rod Labs',
      networkSeed: '098rc1m-09384u-crm-29384u-cmkj',
      icon: {
        type: 'filesystem',
        path: './we_dev/lrl-icon.png',
      },
      creatingAgent: {
        agentIdx: 1,
        agentProfile: {
          nickname: 'Zippy',
          avatar: {
            type: 'filesystem',
            path: './we_dev/zippy.jpg',
          },
        },
      },
      joiningAgents: [
        {
          agentIdx: 2,
          agentProfile: {
            nickname: 'Zerbina',
            avatar: {
              type: 'filesystem',
              path: './we_dev/zerbina.jpg',
            },
          },
        },
      ],
      applets: [
        {
          name: 'Whosin Hot Reload',
          instanceName: 'Whosin Hot Reload',
          registeringAgent: 1,
          joiningAgents: [2],
        },
        // {
        //   name: 'Converge',
        //   instanceName: 'Converge',
        //   registeringAgent: 1,
        //   joiningAgents: [2],
        // },
        // {
        //   name: 'talking-stickies',
        //   instanceName: 'talking-stickies',
        //   registeringAgent: 1,
        //   joiningAgents: [2],
        // },
      ],
    },
  ],
  applets: [
    {
      name: 'Whosin Hot Reload',
      subtitle: 'Whosin Hot Reload',
      description: 'whosin',
      icon: {
        type: 'filesystem',
        path: './we_dev/whosin.png',
      },
      source: {
        type: 'localhost',
        happPath: './workdir/whosin.happ',
        uiPort: 8888,
      },
    },
    {
        name: 'Converge',
        subtitle: 'Decide stuff with others!',
        description: 'Real-time games based on syn',
        icon: {
          type: "https",
          url: "https://raw.githubusercontent.com/lightningrodlabs/converge/273cba658883f5d9cc866d88980d45e8453d3fda/we_dev/converge.svg"
        },
        source: {
          type: "https",
          url: "https://github.com/lightningrodlabs/converge/releases/download/0.0.6/converge.webhapp"
        },
      },
      {
      name: 'talking-stickies',
      subtitle: 'talking stickies',
      description: 'Real-time stickies based on syn',
      icon: {
        type: 'https',
        url: 'https://raw.githubusercontent.com/holochain-apps/talking-stickies/main/we_dev/talking-stickies_icon.png',
      },
      source: {
        type: 'https',
        url: 'https://github.com/holochain-apps/talking-stickies/releases/download/v0.9.1/talking-stickies.webhapp',
      },
    },
  ],
});