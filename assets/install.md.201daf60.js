import{_ as s,c as n,o as a,a as o}from"./app.e3ba1e82.js";const u=JSON.parse('{"title":"Installation","description":"","frontmatter":{"title":"Installation","lang":"en-US"},"headers":[{"level":2,"title":"Get The App Exutable","slug":"get-the-app-exutable","link":"#get-the-app-exutable","children":[{"level":3,"title":"No Brainer Way","slug":"no-brainer-way","link":"#no-brainer-way","children":[]},{"level":3,"title":"Chad Way","slug":"chad-way","link":"#chad-way","children":[]}]},{"level":2,"title":"How to edit config.json","slug":"how-to-edit-config-json","link":"#how-to-edit-config-json","children":[]}],"relativePath":"install.md"}'),l={name:"install.md"},p=o(`<h1 id="how-to-install" tabindex="-1">How To Install <a class="header-anchor" href="#how-to-install" aria-hidden="true">#</a></h1><p>for installation you need to get exutable that could run on your server, Then you need to fill the config.json to run the bot with your config.</p><h2 id="get-the-app-exutable" tabindex="-1">Get The App Exutable <a class="header-anchor" href="#get-the-app-exutable" aria-hidden="true">#</a></h2><p>There is two ways to install this preject into your server.</p><h3 id="no-brainer-way" tabindex="-1">No Brainer Way <a class="header-anchor" href="#no-brainer-way" aria-hidden="true">#</a></h3><p>This way is easiest way but only works if platform you use is already covered on github release.<br></p><ol><li>download the release match your platform</li><li>extract the zip</li><li>fill out <code>config.json</code></li><li>run the exutable</li></ol><h3 id="chad-way" tabindex="-1">Chad Way <a class="header-anchor" href="#chad-way" aria-hidden="true">#</a></h3><p>if you want build or edit from source, or your platform isnt covered on release. <br> you need following tool isntalled on your OS</p><ul><li>Rust latest stable release</li><li>Toolchain for your target os</li><li>git</li><li>curl</li></ul><p>for windows you will need to have <code>linker</code> to build rust project <br> detail of how to install rust and cargo is covered in rust book <a href="https://doc.rust-lang.org/book/ch01-01-installation.html" target="_blank" rel="noreferrer">here</a><br><br></p><p>If you have mentioned tools installed then you can follow this step to install</p><ol><li>download any release you want (its already contain source code)</li><li>extract the zip</li><li>build release the source and place it on main folder <br><br> for <code>MacOs</code> or <code>linux</code></li></ol><div class="language-shell"><button title="Copy Code" class="copy"></button><span class="lang">shell</span><pre class="shiki material-theme-palenight" tabindex="0"><code><span class="line"><span style="color:#FFCB6B;">cargo</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">build</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">--release</span></span>
<span class="line"><span style="color:#FFCB6B;">cp</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">./target/release/rain-bot</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">.</span></span>
<span class="line"></span></code></pre></div><p>for <code>windows</code></p><div class="language-shell"><button title="Copy Code" class="copy"></button><span class="lang">shell</span><pre class="shiki material-theme-palenight" tabindex="0"><code><span class="line"><span style="color:#FFCB6B;">cargo</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">build</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">--release</span></span>
<span class="line"><span style="color:#FFCB6B;">copy</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">./target/release/rain-bot.exe</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">.</span></span>
<span class="line"></span></code></pre></div><h2 id="how-to-edit-config-json" tabindex="-1">How to edit config.json <a class="header-anchor" href="#how-to-edit-config-json" aria-hidden="true">#</a></h2><p>there is something that need to be filled before running the bot <br></p><div class="language-json"><button title="Copy Code" class="copy"></button><span class="lang">json</span><pre class="shiki material-theme-palenight" tabindex="0"><code><span class="line"><span style="color:#89DDFF;">{</span></span>
<span class="line"><span style="color:#A6ACCD;">  </span><span style="color:#89DDFF;">&quot;</span><span style="color:#C792EA;">discord</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">:</span><span style="color:#A6ACCD;"> </span><span style="color:#89DDFF;">{</span></span>
<span class="line"><span style="color:#A6ACCD;">    </span><span style="color:#89DDFF;">&quot;</span><span style="color:#FFCB6B;">token</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">:</span><span style="color:#89DDFF;">&quot;</span><span style="color:#C3E88D;">Your Token</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">,</span></span>
<span class="line"><span style="color:#A6ACCD;">    </span><span style="color:#89DDFF;">&quot;</span><span style="color:#FFCB6B;">prefix</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">:</span><span style="color:#89DDFF;">&quot;</span><span style="color:#C3E88D;">?</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">,</span></span>
<span class="line"><span style="color:#A6ACCD;">    </span><span style="color:#89DDFF;">&quot;</span><span style="color:#FFCB6B;">author_id</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">:</span><span style="color:#F78C6C;">455622761168109569</span></span>
<span class="line"><span style="color:#A6ACCD;">   </span><span style="color:#89DDFF;">},</span></span>
<span class="line"><span style="color:#A6ACCD;">  </span><span style="color:#89DDFF;">&quot;</span><span style="color:#C792EA;">postgress</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">:</span><span style="color:#A6ACCD;"> </span><span style="color:#89DDFF;">{</span></span>
<span class="line"><span style="color:#A6ACCD;">    </span><span style="color:#89DDFF;">&quot;</span><span style="color:#FFCB6B;">user</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">:</span><span style="color:#89DDFF;">&quot;</span><span style="color:#C3E88D;">postgres</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">,</span></span>
<span class="line"><span style="color:#A6ACCD;">    </span><span style="color:#89DDFF;">&quot;</span><span style="color:#FFCB6B;">host</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">:</span><span style="color:#89DDFF;">&quot;</span><span style="color:#C3E88D;">localhost</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">,</span></span>
<span class="line"><span style="color:#A6ACCD;">    </span><span style="color:#89DDFF;">&quot;</span><span style="color:#FFCB6B;">password</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">:</span><span style="color:#89DDFF;">&quot;&quot;</span><span style="color:#89DDFF;">,</span></span>
<span class="line"><span style="color:#A6ACCD;">    </span><span style="color:#89DDFF;">&quot;</span><span style="color:#FFCB6B;">port</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">:</span><span style="color:#F78C6C;">6432</span><span style="color:#89DDFF;">,</span></span>
<span class="line"><span style="color:#A6ACCD;">    </span><span style="color:#89DDFF;">&quot;</span><span style="color:#FFCB6B;">database</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">:</span><span style="color:#89DDFF;">&quot;</span><span style="color:#C3E88D;">erupe</span><span style="color:#89DDFF;">&quot;</span></span>
<span class="line"><span style="color:#A6ACCD;">   </span><span style="color:#89DDFF;">},</span></span>
<span class="line"><span style="color:#A6ACCD;">  </span><span style="color:#89DDFF;">&quot;</span><span style="color:#C792EA;">mhfz_config</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">:</span><span style="color:#A6ACCD;"> </span><span style="color:#89DDFF;">{</span><span style="color:#A6ACCD;">   </span></span>
<span class="line"><span style="color:#A6ACCD;">    </span><span style="color:#89DDFF;">&quot;</span><span style="color:#FFCB6B;">account_creation</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">:true,</span></span>
<span class="line"><span style="color:#A6ACCD;">    </span><span style="color:#89DDFF;">&quot;</span><span style="color:#FFCB6B;">sending_log</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">:false</span></span>
<span class="line"><span style="color:#A6ACCD;">   </span><span style="color:#89DDFF;">},</span></span>
<span class="line"><span style="color:#A6ACCD;">  </span><span style="color:#89DDFF;">&quot;</span><span style="color:#C792EA;">bot_config</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">:</span><span style="color:#A6ACCD;"> </span><span style="color:#89DDFF;">{</span></span>
<span class="line"><span style="color:#A6ACCD;">    </span><span style="color:#89DDFF;">&quot;</span><span style="color:#FFCB6B;">member_join</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">:true,</span></span>
<span class="line"><span style="color:#A6ACCD;">    </span><span style="color:#89DDFF;">&quot;</span><span style="color:#FFCB6B;">member_leave</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">:true,</span></span>
<span class="line"><span style="color:#A6ACCD;">    </span><span style="color:#89DDFF;">&quot;</span><span style="color:#FFCB6B;">gacha</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">:true,</span></span>
<span class="line"><span style="color:#A6ACCD;">    </span><span style="color:#89DDFF;">&quot;</span><span style="color:#FFCB6B;">bounty</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">:true,</span></span>
<span class="line"><span style="color:#A6ACCD;">    </span><span style="color:#89DDFF;">&quot;</span><span style="color:#FFCB6B;">server_market</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">:true</span></span>
<span class="line"><span style="color:#A6ACCD;">   </span><span style="color:#89DDFF;">},</span></span>
<span class="line"><span style="color:#A6ACCD;">  </span><span style="color:#89DDFF;">&quot;</span><span style="color:#C792EA;">log_channel</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">:</span><span style="color:#A6ACCD;"> </span><span style="color:#89DDFF;">{</span></span>
<span class="line"><span style="color:#A6ACCD;">    </span><span style="color:#89DDFF;">&quot;</span><span style="color:#FFCB6B;">err_channel</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">:</span><span style="color:#F78C6C;">1031774270512169070</span><span style="color:#89DDFF;">,</span></span>
<span class="line"><span style="color:#A6ACCD;">    </span><span style="color:#89DDFF;">&quot;</span><span style="color:#FFCB6B;">account_channel</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">:</span><span style="color:#F78C6C;">1031774270512169070</span><span style="color:#89DDFF;">,</span></span>
<span class="line"><span style="color:#A6ACCD;">    </span><span style="color:#89DDFF;">&quot;</span><span style="color:#FFCB6B;">transfer_channel</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">:</span><span style="color:#F78C6C;">1031774270512169070</span><span style="color:#89DDFF;">,</span></span>
<span class="line"><span style="color:#A6ACCD;">    </span><span style="color:#89DDFF;">&quot;</span><span style="color:#FFCB6B;">info_channel</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">:</span><span style="color:#F78C6C;">1031774270512169070</span><span style="color:#89DDFF;">,</span></span>
<span class="line"><span style="color:#A6ACCD;">    </span><span style="color:#89DDFF;">&quot;</span><span style="color:#FFCB6B;">info_channel_msg</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">:</span><span style="color:#F78C6C;">1077951127317921874</span><span style="color:#89DDFF;">,</span></span>
<span class="line"><span style="color:#A6ACCD;">    </span><span style="color:#89DDFF;">&quot;</span><span style="color:#FFCB6B;">market_channel</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">:</span><span style="color:#F78C6C;">1031774270512169070</span><span style="color:#89DDFF;">,</span></span>
<span class="line"><span style="color:#A6ACCD;">    </span><span style="color:#89DDFF;">&quot;</span><span style="color:#FFCB6B;">market_channel_msg</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">:</span><span style="color:#F78C6C;">1076848380971589634</span><span style="color:#89DDFF;">,</span></span>
<span class="line"><span style="color:#A6ACCD;">    </span><span style="color:#89DDFF;">&quot;</span><span style="color:#FFCB6B;">erupe_channel</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">:</span><span style="color:#F78C6C;">1031774270512169070</span><span style="color:#89DDFF;">,</span></span>
<span class="line"><span style="color:#A6ACCD;">    </span><span style="color:#89DDFF;">&quot;</span><span style="color:#FFCB6B;">request_channel</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">:</span><span style="color:#F78C6C;">1031774270512169070</span><span style="color:#89DDFF;">,</span></span>
<span class="line"><span style="color:#A6ACCD;">    </span><span style="color:#89DDFF;">&quot;</span><span style="color:#FFCB6B;">join_channel</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">:</span><span style="color:#F78C6C;">1031774270512169070</span><span style="color:#89DDFF;">,</span></span>
<span class="line"><span style="color:#A6ACCD;">    </span><span style="color:#89DDFF;">&quot;</span><span style="color:#FFCB6B;">leave_channel</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">:</span><span style="color:#F78C6C;">1031774270512169070</span></span>
<span class="line"><span style="color:#A6ACCD;">   </span><span style="color:#89DDFF;">},</span></span>
<span class="line"><span style="color:#A6ACCD;">  </span><span style="color:#89DDFF;">&quot;</span><span style="color:#C792EA;">server_role</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">:{</span></span>
<span class="line"><span style="color:#A6ACCD;">    </span><span style="color:#89DDFF;">&quot;</span><span style="color:#FFCB6B;">admin_role</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">:</span><span style="color:#F78C6C;">1031595216538452038</span><span style="color:#89DDFF;">,</span></span>
<span class="line"><span style="color:#A6ACCD;">    </span><span style="color:#89DDFF;">&quot;</span><span style="color:#FFCB6B;">bartender_role</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">:</span><span style="color:#F78C6C;">1017643913667936318</span><span style="color:#89DDFF;">,</span></span>
<span class="line"><span style="color:#A6ACCD;">    </span><span style="color:#89DDFF;">&quot;</span><span style="color:#FFCB6B;">member_role</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">:</span><span style="color:#F78C6C;">0</span><span style="color:#89DDFF;">,</span></span>
<span class="line"><span style="color:#A6ACCD;">    </span><span style="color:#89DDFF;">&quot;</span><span style="color:#FFCB6B;">register_role</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">:</span><span style="color:#F78C6C;">1017643913667936318</span><span style="color:#89DDFF;">,</span></span>
<span class="line"><span style="color:#A6ACCD;">    </span><span style="color:#89DDFF;">&quot;</span><span style="color:#FFCB6B;">judge_role</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">:</span><span style="color:#F78C6C;">1017643913667936318</span><span style="color:#89DDFF;">,</span></span>
<span class="line"><span style="color:#A6ACCD;">    </span><span style="color:#89DDFF;">&quot;</span><span style="color:#FFCB6B;">maintainer_role</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">:</span><span style="color:#F78C6C;">1017643913667936318</span></span>
<span class="line"><span style="color:#A6ACCD;">  </span><span style="color:#89DDFF;">},</span></span>
<span class="line"><span style="color:#A6ACCD;">  </span><span style="color:#89DDFF;">&quot;</span><span style="color:#C792EA;">bounty</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">:</span><span style="color:#A6ACCD;"> </span><span style="color:#89DDFF;">{</span></span>
<span class="line"><span style="color:#A6ACCD;">    </span><span style="color:#89DDFF;">&quot;</span><span style="color:#FFCB6B;">board_ch</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">:</span><span style="color:#F78C6C;">1031774270512169070</span><span style="color:#89DDFF;">,</span></span>
<span class="line"><span style="color:#A6ACCD;">    </span><span style="color:#89DDFF;">&quot;</span><span style="color:#FFCB6B;">conquered_ch</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">:</span><span style="color:#F78C6C;">1031774270512169070</span><span style="color:#89DDFF;">,</span></span>
<span class="line"><span style="color:#A6ACCD;">    </span><span style="color:#89DDFF;">&quot;</span><span style="color:#FFCB6B;">promotion_ch</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">:</span><span style="color:#F78C6C;">1031774270512169070</span><span style="color:#89DDFF;">,</span></span>
<span class="line"><span style="color:#A6ACCD;">    </span><span style="color:#89DDFF;">&quot;</span><span style="color:#FFCB6B;">cooldown_ch</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">:</span><span style="color:#F78C6C;">1031774270512169070</span><span style="color:#89DDFF;">,</span></span>
<span class="line"><span style="color:#A6ACCD;">    </span><span style="color:#89DDFF;">&quot;</span><span style="color:#FFCB6B;">cooldown_msg</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">:</span><span style="color:#F78C6C;">1081347237566750740</span><span style="color:#89DDFF;">,</span></span>
<span class="line"><span style="color:#A6ACCD;">    </span><span style="color:#89DDFF;">&quot;</span><span style="color:#FFCB6B;">leaderboard_ch</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">:</span><span style="color:#F78C6C;">1031774270512169070</span><span style="color:#89DDFF;">,</span></span>
<span class="line"><span style="color:#A6ACCD;">    </span><span style="color:#89DDFF;">&quot;</span><span style="color:#FFCB6B;">receptionist_ch</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">:</span><span style="color:#F78C6C;">1031774270512169070</span><span style="color:#89DDFF;">,</span></span>
<span class="line"><span style="color:#A6ACCD;">    </span><span style="color:#89DDFF;">&quot;</span><span style="color:#FFCB6B;">judge_ch</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">:</span><span style="color:#F78C6C;">1031774270512169070</span></span>
<span class="line"><span style="color:#A6ACCD;">   </span><span style="color:#89DDFF;">}</span></span>
<span class="line"><span style="color:#89DDFF;">}</span></span>
<span class="line"></span></code></pre></div><p>The <code>discord</code> and <code>postgress</code> section is self explained, you must fill them properly,<br> As for <code>mhfz_config</code> fill the one that matched your server configuration, the send log is working but experimentall, it will be explained further in <a href="/Rain-rust-bot/guide/server.html">Server Monitor</a>,<br> Then <code>bot_config</code> is to enable and disable bot features, set them to match your prefferences,<br> The <code>log_channel</code> section consist the channel the bot will operate based on config, so some can be ignored if you set it false in<code>bot_config</code>, The <code>server_role</code> section consist role that bot will give access command to, <code>admin</code> for admin command,<code>register</code> to give register role and <code>judge</code> for save file judge is the must filled field here,<br> Lastly <code>bounty</code> section can be ignored if you set bounty field on <code>bot_config</code> to false</p>`,20),e=[p];function t(r,c,F,D,y,i){return a(),n("div",null,e)}const d=s(l,[["render",t]]);export{u as __pageData,d as default};
