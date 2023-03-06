import{_ as e,c as t,o as a,a as n}from"./app.e3ba1e82.js";const p=JSON.parse('{"title":"Trading Guide","description":"","frontmatter":{"title":"Trading Guide","lang":"en-US"},"headers":[{"level":2,"title":"Market","slug":"market","link":"#market","children":[]},{"level":2,"title":"Restourant","slug":"restourant","link":"#restourant","children":[]},{"level":2,"title":"Bar","slug":"bar","link":"#bar","children":[]},{"level":2,"title":"Casino","slug":"casino","link":"#casino","children":[]},{"level":2,"title":"Guild","slug":"guild","link":"#guild","children":[]},{"level":2,"title":"Jewelry","slug":"jewelry","link":"#jewelry","children":[]}],"relativePath":"guide/trading.md"}'),i={name:"guide/trading.md"},o=n('<h1 id="trading-commands" tabindex="-1">Trading Commands <a class="header-anchor" href="#trading-commands" aria-hidden="true">#</a></h1><p>trading command is the new feature introduced in this bot, there is some section that will be covered in this page, but generally trading command is grouped command used for trading between player and server, player can get discount on every trading command except the bar using <code>Title benefit</code> gained trough <a href="./bounty.html">Bounty</a>, to enable this command you need to set <code>server-market</code> to true on json config mentioned on <a href="./../install.html">Installation Guide</a>. while there is 6 command total you could setting it so some are disabled or enabled trough <a href="./../config/trading.html">Trading Config</a></p><h2 id="market" tabindex="-1">Market <a class="header-anchor" href="#market" aria-hidden="true">#</a></h2><p>this command using <code>/trading market</code>, the field <code>market_channel</code> and <code>market_channel_msg</code> on <a href="./../install.html">Installation Guide</a> need to be filled in order this command to fully functional, this command is for player so they could buy listed item on the <a href="./../config/trading.html">Config File</a>, each item had stock so you need to chnage the config to restock or change the item list, player can select item they wanted and input quantity they want to buy, the player given the <code>receipt</code> of their transaction including details of what they buy and the price, discount also listed if they had, then player can press <code>confirm</code> to confirm their transaction or <code>cencel</code> to cencel it.</p><h2 id="restourant" tabindex="-1">Restourant <a class="header-anchor" href="#restourant" aria-hidden="true">#</a></h2><p>this command using <code>/trading restourant</code>, the fuction is for the player to buy guild food for for guilds they are on, the price is depend on the duration the food lasted, the price can be set by <a href="./../config/trading.html">Config</a> and the menu list can be change or updated trough meal section covered in <a href="./../config/trading.html">Config</a> but i sugested just keep default config by Ten from Rain server. like market command player will be promted with <code>Receipt</code> to confirm their transaction.</p><h2 id="bar" tabindex="-1">Bar <a class="header-anchor" href="#bar" aria-hidden="true">#</a></h2><p>this command using <code>/trading bar</code>, the function is for player to post their request, player can type the items/equipment they want also filled the rerquested price to <code>bartender_role</code>, those request will be posted on the <code>request_channel</code> new thread and both bartender and player could discus togetheir regarding their request, btw since this is custom request player discount is inactive and admin could send item/equipments to player with <code>/send</code> command (those command is fixed and player will be charged correctly).</p><h2 id="casino" tabindex="-1">Casino <a class="header-anchor" href="#casino" aria-hidden="true">#</a></h2><p>this command use <code>/trading casino</code>, this one used to buy gacha ticket for <code>/pull</code> command, the price can be adjusted on <a href="./../config/trading.html">Trading Config</a>. this command also promp <code>Receipt</code></p><h2 id="guild" tabindex="-1">Guild <a class="header-anchor" href="#guild" aria-hidden="true">#</a></h2><p>this command use <code>/trading guild</code>, this one used to buy guid RP for guild player currently in, the price can be adjusted on <a href="./../config/trading.html">Trading Config</a>. this command also promp <code>Receipt</code></p><h2 id="jewelry" tabindex="-1">Jewelry <a class="header-anchor" href="#jewelry" aria-hidden="true">#</a></h2><p>this command use <code>/trading jewelry</code>, this one used to buy <code>gacha premium</code> used for upgrading caravan gem, skiping my mission or gacha cat ingame , the price can be adjusted on <a href="./../config/trading.html">Trading Config</a>. this command also promp <code>Receipt</code></p>',14),d=[o];function r(c,s,l,h,u,m){return a(),t("div",null,d)}const f=e(i,[["render",r]]);export{p as __pageData,f as default};
