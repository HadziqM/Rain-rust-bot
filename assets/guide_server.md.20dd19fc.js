import{_ as e,c as o,o as s,d as t}from"./app.21480e27.js";const y=JSON.parse('{"title":"Server Monitor","description":"","frontmatter":{"title":"Server Monitor","lang":"en-US"},"headers":[{"level":2,"title":"About","slug":"about","link":"#about","children":[]},{"level":2,"title":"History","slug":"history","link":"#history","children":[]},{"level":2,"title":"How to use","slug":"how-to-use","link":"#how-to-use","children":[{"level":3,"title":"Log File","slug":"log-file","link":"#log-file","children":[]}]}],"relativePath":"guide/server.md"}'),a={name:"guide/server.md"},n=t(`<h1 id="server-monitor" tabindex="-1">Server Monitor <a class="header-anchor" href="#server-monitor" aria-hidden="true">#</a></h1><h2 id="about" tabindex="-1">About <a class="header-anchor" href="#about" aria-hidden="true">#</a></h2><p>One of feature bot had is to monitor mhfz server status also sent its log file, this work by looking into database each minutes then send its result in <code>info_channel</code> channel editing <code>info_channel_msg</code>, so those config covered in <a href="https:/hadziqm.github.io/Rain-rust-bot/install.html">Installation</a> must be filled for bot to run.</p><h2 id="history" tabindex="-1">History <a class="header-anchor" href="#history" aria-hidden="true">#</a></h2><p>This feature added because Erupe is still under development, in case server crash, bot will inform you and send the log if <code>send_log</code> on <a href="https:/hadziqm.github.io/Rain-rust-bot/install.html">Installation</a> section is enabled this will helpfull for Erupe developer to analyze the server issues, since Erupe still doesnt have away to log its process into file, there is some bash command to capture <code>Stdout</code> and <code>StdErr</code> of Erupe while on run, the file could exceed 4GB (on experiment done by Rain Server) in a day using that bash command, The bot could send and reset the log file in every 5 minutes to avoid populating server memory (still on test) if the <code>send_log</code> enabled (i suggest you disabled it untill further notification).</p><h2 id="how-to-use" tabindex="-1">How to use <a class="header-anchor" href="#how-to-use" aria-hidden="true">#</a></h2><p>The monitor is enabled by default so you can get pinged on crash notification if you fill <code>maintainer_role</code> on <a href="https:/hadziqm.github.io/Rain-rust-bot/install.html">Installation</a>. but you could disabled it by using <code>/monitor</code> command if youare server admin, and use it again to anabled it back.</p><h3 id="log-file" tabindex="-1">Log File <a class="header-anchor" href="#log-file" aria-hidden="true">#</a></h3><p>First you need to make <code>log.txt</code> in bot folder for bot to send a file, those only placeholder for actual log.<br> Then on the erupe folder you need to use this command to log file while running erupe</p><div class="language-shell"><button title="Copy Code" class="copy"></button><span class="lang">shell</span><pre class="shiki material-theme-palenight" tabindex="0"><code><span class="line"><span style="color:#FFCB6B;">erupe-ce</span><span style="color:#A6ACCD;"> </span><span style="color:#89DDFF;">2&gt;&amp;1</span><span style="color:#A6ACCD;"> </span><span style="color:#89DDFF;">|</span><span style="color:#A6ACCD;"> </span><span style="color:#FFCB6B;">tee</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">-a</span><span style="color:#A6ACCD;"> </span><span style="color:#89DDFF;">&lt;</span><span style="color:#C3E88D;">pat</span><span style="color:#A6ACCD;">h</span><span style="color:#89DDFF;">&gt;</span><span style="color:#C3E88D;">/log.txt</span></span>
<span class="line"></span></code></pre></div><p>the path its mentioned is the bot&#39;s forder path, so for example you can use this if bot&#39;s folder is inside erupe folder</p><div class="language-shell"><button title="Copy Code" class="copy"></button><span class="lang">shell</span><pre class="shiki material-theme-palenight" tabindex="0"><code><span class="line"><span style="color:#FFCB6B;">erupe-ce</span><span style="color:#A6ACCD;"> </span><span style="color:#89DDFF;">2&gt;&amp;1</span><span style="color:#A6ACCD;"> </span><span style="color:#89DDFF;">|</span><span style="color:#A6ACCD;"> </span><span style="color:#FFCB6B;">tee</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">-a</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">./Rain-rust-bot/log.txt</span></span>
<span class="line"></span></code></pre></div><div class="warning custom-block"><p class="custom-block-title">WARNING</p><p>for windows, the path separator will use <code>\\</code> instead <code>/</code> for unix</p></div>`,13),l=[n];function i(r,d,c,p,h,u){return s(),o("div",null,l)}const g=e(a,[["render",i]]);export{y as __pageData,g as default};