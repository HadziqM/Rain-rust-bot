export default {
  title: 'Rain Bot',
  description: 'The documentation for rain bot',
  base: 'https://hadziqm.github.io/Rain-rust-bot/',
  themeConfig: {
    logo:"https://media.discordapp.net/attachments/1021963806315905085/1059685171835699210/Rain_Server_Logo.png?width=693&height=658",
    socialLinks:[
      { icon: "github", link: "https://github.com/HadziqM/Rain-rust-bot" },
      { icon: "discord", link: "https://discord.gg/GRQHbMajMy" },
    ],
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Installation', link: '/install' },
            { 
        text: 'Guide',
        items:[
          {text:"Most Command",link:'/guide/'},
          {text:"Server Monitor",link:'/guide/server'},
          {text:"Register Command",link:'/guide/register'},
          {text:"Trading Command",link:'/guide/trading'},
          {text:"Bounty Command",link:'/guide/bounty'},
          {text:"Json Config",link:'/guide/json'},
          {text:"Custom Query",link:'/guide/query'},
          {text:"Tag Command",link:'/guide/tag'},
        ]},
      { 
        text: 'Config',
        items:[
          {text:"Bounty",link:'/config/bounty'},
          {text:"Gacha",link:'/config/gacha'},
          {text:"Market",link:'/config/market'},
          {text:"Meals",link:'/config/meals'},
          {text:"Tag",link:'/config/tag'},
          {text:"Title",link:'/config/title'},
          {text:"Trading",link:'/config/trading'},
        ]},
    ],
  },
};
