(()=>{var y=null;function J(o){return`nx-${o}-${Math.random().toString(36).substring(2,8)}`}function W(o,r){if(!o||!r)return;let d=o.replace(/([^{}]+){/g,(b,a)=>{let n=a.trim();if(n.startsWith("@")||n.includes(`[data-scope="${r}"]`))return b;return`${n.split(",").map((u)=>{let p=u.trim();return`[data-scope="${r}"] ${p}`}).join(", ")} {`}),x=document.createElement("style");x.textContent=d,x.setAttribute("data-scope-id",r),document.head.appendChild(x)}function ko(o,r,...d){if(typeof o==="function"){if(o&&typeof o==="object"&&"component"in o&&"css"in o){let b=o,a=b.component.name||"Component",n=J(a);W(b.css,n);let f=y;y=n;let u=b.component({...r,children:d});return y=f,u}if(o.css){let b=o.name||"Component",a=J(b);W(o.css,a);let n=y;y=a;let f=o({...r,children:d});return y=n,f}return o({...r,children:d})}let x=document.createElement(o);if(y)x.setAttribute("data-scope",y);if(r)Object.entries(r).forEach(([b,a])=>{if(b==="className")x.className=a;else if(b==="css"){let n=J("inline");W(a,n),x.setAttribute("data-scope",n)}else if(b.startsWith("on")&&typeof a==="function"){let n=b.toLowerCase().slice(2);x.addEventListener(n,a)}else x.setAttribute(b,a)});return d.flat().forEach((b)=>{if(typeof b==="string"||typeof b==="number")x.appendChild(document.createTextNode(String(b)));else if(b instanceof Node)x.appendChild(b)}),x}function zo({children:o}){let r=document.createDocumentFragment();return o.flat().forEach((d)=>{if(typeof d==="string"||typeof d==="number")r.appendChild(document.createTextNode(String(d)));else if(d instanceof Node)r.appendChild(d)}),r}globalThis.createElement=ko;globalThis.Fragment=zo;var c=`.ns-card {
  background: var(--color-surface, #ffffff);
  border: 1px solid var(--color-border, #e1e5e9);
  border-radius: var(--border-radius-lg, 8px);
  box-shadow: var(--shadow-sm, 0 1px 3px rgba(0, 0, 0, 0.1));
  overflow: hidden;
  transition: box-shadow var(--transition-fast, 0.15s) ease;
}

.ns-card:hover {
  box-shadow: var(--shadow-md, 0 4px 6px rgba(0, 0, 0, 0.1));
}

.ns-card-header {
  padding: var(--spacing-lg, 16px) var(--spacing-xl, 20px);
  border-bottom: 1px solid var(--color-border, #e1e5e9);
  background: var(--color-surface-elevated, #f8f9fa);
}

.ns-card-title {
  margin: 0;
  font-size: var(--font-size-xl, 18px);
  font-weight: 600;
  color: var(--text-primary, #2d3748);
}

.ns-card-subtitle {
  margin: 4px 0 0 0;
  font-size: var(--font-size-base, 14px);
  color: var(--text-secondary, #718096);
}

.ns-card-body {
  padding: var(--spacing-xl, 20px);
}

.ns-card-footer {
  padding: var(--spacing-md, 12px) var(--spacing-xl, 20px);
  border-top: 1px solid var(--color-border, #e1e5e9);
  background: var(--color-surface-elevated, #f8f9fa);
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--spacing-md, 12px);
}

/* Variants */
.ns-card.elevated {
  box-shadow: var(--shadow-lg, 0 10px 15px rgba(0, 0, 0, 0.1));
}

.ns-card.flat {
  box-shadow: none;
  border: 1px solid var(--color-border, #e1e5e9);
}

.ns-card.borderless {
  border: none;
  box-shadow: none;
}`;function K(o){let{children:r,title:d,subtitle:x,footer:b,variant:a="default",className:n=""}=o,f=["ns-card",a!=="default"&&a,n].filter(Boolean).join(" ");return createElement("div",{className:f},(d||x)&&createElement("div",{className:"ns-card-header"},d&&createElement("h3",{className:"ns-card-title"},d),x&&createElement("p",{className:"ns-card-subtitle"},x)),r&&createElement("div",{className:"ns-card-body"},r),b&&createElement("div",{className:"ns-card-footer"},b))}K.css=c;var B=K;var L=`.ns-badge {
  display: inline-flex;
  align-items: center;
  padding: 2px 8px;
  border-radius: 12px;
  font-size: 12px;
  font-weight: 500;
  line-height: 1.4;
  text-transform: uppercase;
  letter-spacing: 0.025em;
  border: 1px solid transparent;
}

/* Variants */
.ns-badge.success {
  background-color: var(--success-bg, #d1fae5);
  color: var(--success-color, #065f46);
  border-color: var(--success-border, #a7f3d0);
}

.ns-badge.warning {
  background-color: var(--warning-bg, #fef3c7);
  color: var(--warning-color, #92400e);
  border-color: var(--warning-border, #fde68a);
}

.ns-badge.error {
  background-color: var(--error-bg, #fee2e2);
  color: var(--error-color, #991b1b);
  border-color: var(--error-border, #fecaca);
}

.ns-badge.info {
  background-color: var(--info-bg, #dbeafe);
  color: var(--info-color, #1e40af);
  border-color: var(--info-border, #bfdbfe);
}

.ns-badge.neutral {
  background-color: var(--neutral-bg, #f3f4f6);
  color: var(--neutral-color, #374151);
  border-color: var(--neutral-border, #d1d5db);
}

/* Sizes */
.ns-badge.small {
  padding: 1px 6px;
  font-size: 10px;
}

.ns-badge.large {
  padding: 4px 12px;
  font-size: 14px;
  border-radius: 16px;
}

/* Styles */
.ns-badge.outline {
  background-color: transparent;
}

.ns-badge.solid {
  border-color: transparent;
}

.ns-badge.solid.success {
  background-color: var(--success-solid-bg, #059669);
  color: white;
}

.ns-badge.solid.warning {
  background-color: var(--warning-solid-bg, #d97706);
  color: white;
}

.ns-badge.solid.error {
  background-color: var(--error-solid-bg, #dc2626);
  color: white;
}

.ns-badge.solid.info {
  background-color: var(--info-solid-bg, #2563eb);
  color: white;
}

.ns-badge.solid.neutral {
  background-color: var(--neutral-solid-bg, #6b7280);
  color: white;
}`;function R(o){let{children:r,variant:d="neutral",size:x="medium",style:b="default",className:a=""}=o,n=["ns-badge",d,x!=="medium"&&x,b!=="default"&&b,a].filter(Boolean).join(" ");return createElement("span",{className:n},r)}R.css=L;var F=R;var A=`.ns-button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  padding: var(--spacing-sm, 8px) var(--spacing-lg, 16px);
  border: 1px solid transparent;
  border-radius: var(--border-radius-md, 6px);
  cursor: pointer;
  font-weight: 500;
  font-size: var(--font-size-base, 14px);
  line-height: 1.4;
  text-decoration: none;
  transition: all var(--transition-fast, 0.15s) ease;
  user-select: none;
  min-height: 36px;
  gap: 6px;
  font-family: var(--font-family, inherit);
}

.ns-button:focus {
  outline: 2px solid var(--primary, #0070f3);
  outline-offset: 2px;
}

.ns-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
  pointer-events: none;
}

/* Variants */
.ns-button.primary {
  background-color: var(--primary, #0070f3);
  color: var(--text-inverse, white);
  border-color: var(--primary, #0070f3);
}

.ns-button.primary:hover:not(:disabled) {
  background-color: var(--primary-hover, #0061d5);
  border-color: var(--primary-hover, #0061d5);
  transform: translateY(-1px);
}

.ns-button.secondary {
  background-color: var(--color-surface-elevated, #f8f9fa);
  color: var(--text-secondary, #666);
  border-color: var(--color-border, #e1e5e9);
}

.ns-button.secondary:hover:not(:disabled) {
  background-color: var(--color-surface-hover, #f5f5f5);
  color: var(--text-primary, #2d3748);
  border-color: var(--color-border, #e1e5e9);
}

.ns-button.danger {
  background-color: var(--danger, #dc3545);
  color: var(--text-inverse, white);
  border-color: var(--danger, #dc3545);
}

.ns-button.danger:hover:not(:disabled) {
  background-color: var(--danger-hover, #c82333);
  border-color: var(--danger-hover, #c82333);
  transform: translateY(-1px);
}

.ns-button.ghost {
  background-color: transparent;
  color: var(--text-secondary, #666);
  border: 1px solid transparent;
}

.ns-button.ghost:hover:not(:disabled) {
  background-color: var(--color-surface-elevated, #f8f9fa);
  color: var(--text-primary, #2d3748);
}

/* Sizes */
.ns-button.small {
  padding: 4px 12px;
  font-size: 12px;
  min-height: 28px;
}

.ns-button.large {
  padding: 12px 24px;
  font-size: 16px;
  min-height: 44px;
}

/* Loading state */
.ns-button.loading {
  position: relative;
  color: transparent;
}

.ns-button.loading::after {
  content: '';
  position: absolute;
  width: 16px;
  height: 16px;
  border: 2px solid currentColor;
  border-radius: 50%;
  border-top-color: transparent;
  animation: spin 0.8s linear infinite;
  color: inherit;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}`;function V(o){let{children:r,variant:d="primary",size:x="medium",disabled:b=!1,loading:a=!1,type:n="button",onClick:f,href:u,target:p,className:yo=""}=o,P=["ns-button",d,x,a&&"loading",yo].filter(Boolean).join(" ");if(u)return createElement("a",{href:u,target:p,className:P,onClick:f,"aria-disabled":b},r);return createElement("button",{type:n,className:P,...b?{disabled:!0}:{},onClick:f},r)}V.css=A;var q=V;function wo(o,r){if(r.css&&!document.querySelector(`style[data-component="${o}"]`)){let x=document.createElement("style");x.textContent=r.css,x.setAttribute("data-component",o),document.head.appendChild(x)}class d extends HTMLElement{mounted=!1;static get observedAttributes(){return r.observedAttributes||[]}connectedCallback(){this.render(),this.mounted=!0}attributeChangedCallback(){if(this.mounted)this.render()}render(){let x={};for(let f=0;f<this.attributes.length;f++){let u=this.attributes[f];if(!u)continue;let p=u.value;if(p.startsWith("{")||p.startsWith("[")||p==="true"||p==="false")try{p=JSON.parse(p)}catch{}x[this.camelCase(u.name)]=p}for(let f in x)if(f.startsWith("on")&&typeof x[f]==="string"){let u=f.slice(2).toLowerCase(),p=x[f];if(p.includes("alert")||p.includes("console"))x[f]=new Function("event",p)}let b=this.innerHTML,a=[];if(b.trim()){let f=document.createElement("div");f.innerHTML=b,a.push(...Array.from(f.childNodes))}if(a.length>0)x.children=a;this.innerHTML="";let n=r.component(x);if(n instanceof Node)this.appendChild(n)}camelCase(x){return x.replace(/-([a-z])/g,(b,a)=>a.toUpperCase())}}customElements.define(o,d)}function H(o,r){wo(o,{component:r,css:r.css,observedAttributes:["*"]})}var g={"ns-card":B,"ns-badge":F,"ns-button":q};function G(){Object.entries(g).forEach(([o,r])=>{H(o,r)}),console.log("Auto-registered components:",Object.keys(g))}function l(o,r="info"){let d=document.createElement("div");d.className=`message message-${r}`,d.textContent=o;let x=document.getElementById("messages-container");if(!x)x=document.createElement("div"),x.id="messages-container",x.className="messages",document.body.appendChild(x);x.appendChild(d),setTimeout(()=>{if(d.parentNode)d.parentNode.removeChild(d)},5000)}function z(){let o=document.querySelector(".modal-overlay");if(o)o.remove()}function h(o){let r=document.getElementById(o);if(!r)return;document.querySelectorAll(".dropdown.active").forEach((d)=>{if(d.id!==o)d.classList.remove("active")}),r.classList.toggle("active")}function w(){document.querySelectorAll(".dropdown.active").forEach((o)=>{o.classList.remove("active")})}var k={SERVICE_CONFIG:(o)=>`nexsock_service_config_${o}`,GIT_CONTENT_COLLAPSED:(o)=>`git_${o}_collapsed`};function U(o,r){document.querySelectorAll(".tab-button").forEach((b)=>{b.classList.remove("active")});let d=event?.target;if(d)d.classList.add("active");let x=document.getElementById("git-tab-content");if(!x)return;if(o==="commits")x.innerHTML='<div class="loading">Loading commits...</div>',window.htmx.ajax("GET",`/api/templates/git-log?service=${r}`,{target:"#git-tab-content",swap:"innerHTML"});else if(o==="branches")x.innerHTML='<div class="loading">Loading branches...</div>',window.htmx.ajax("GET",`/api/templates/git-branches?service=${r}`,{target:"#git-tab-content",swap:"innerHTML"})}function v(o){let r=document.getElementById("new-branch-name");if(!r)return;let d=r.value.trim();if(!d){l("Please enter a branch name","warning");return}if(!confirm(`Create new branch "${d}" and switch to it?`))return;let x=new FormData;x.append("branch",d),x.append("create","true"),fetch(`/api/services/${o}/git/checkout/branch`,{method:"POST",body:x}).then((b)=>{if(!b.ok)throw new Error(`HTTP error: ${b.status}`);return b.json()}).then((b)=>{r.value="",window.htmx.ajax("GET",`/api/templates/git-section?service=${o}`,{target:"#git-section",swap:"outerHTML"}),l(`Successfully created and switched to branch "${d}"`,"success")}).catch((b)=>{console.error("Error creating branch:",b),l("Failed to create branch","error")})}function M(o){window.htmx.ajax("GET",`/api/templates/git-section?service=${o}`,{target:"#git-section",swap:"outerHTML"})}function T(o){let r=document.getElementById(o);if(!r)return;r.classList.toggle("collapsed");let d=r.classList.contains("collapsed");localStorage.setItem(k.GIT_CONTENT_COLLAPSED(o),d.toString())}function t(){let o=localStorage.getItem(k.GIT_CONTENT_COLLAPSED("git-commits-list"))==="true",r=document.getElementById("git-commits-list");if(r&&o)r.classList.add("collapsed");let d=localStorage.getItem(k.GIT_CONTENT_COLLAPSED("git-branches-list"))==="true",x=document.getElementById("git-branches-list");if(x&&d)x.classList.add("collapsed")}var to={light:"☀️",dark:"\uD83C\uDF19",auto:"\uD83D\uDCBB"};class I{currentTheme="auto";isInitialized=!1;constructor(){if(!this.isInitialized)this.initializeTheme(),this.setupThemeToggle(),this.isInitialized=!0}setTheme(o){if(!this.isValidTheme(o)){console.warn(`Invalid theme: ${o}`);return}this.currentTheme=o,localStorage.setItem("nexsock-theme",o),this.applyTheme(),this.updateThemeIcon()}getCurrentTheme(){return this.currentTheme}getEffectiveTheme(){if(this.currentTheme==="auto")return window.matchMedia("(prefers-color-scheme: dark)").matches?"dark":"light";return this.currentTheme}initializeTheme(){let o=localStorage.getItem("nexsock-theme");if(o&&this.isValidTheme(o))this.currentTheme=o;else this.currentTheme="auto";this.applyTheme(),this.updateThemeIcon()}setupThemeToggle(){let o=document.getElementById("theme-toggle");if(o&&!o.hasAttribute("data-theme-listener")){let r=()=>{this.toggleTheme()};o.addEventListener("click",r),o.setAttribute("data-theme-listener","true")}}toggleTheme(){let o=["light","dark","auto"],d=(o.indexOf(this.currentTheme)+1)%o.length,x=o[d];if(x)this.setTheme(x)}applyTheme(){document.documentElement.setAttribute("data-theme",this.currentTheme)}updateThemeIcon(){let o=document.querySelector(".theme-icon");if(o)o.textContent=to[this.currentTheme];let r=document.getElementById("theme-toggle");if(r)r.setAttribute("aria-label",`Current theme: ${this.currentTheme}. Click to change theme.`)}isValidTheme(o){return["light","dark","auto"].includes(o)}}var Z=null;function S(){if(!Z)Z=new I;return Z}function X(){return Z}function Yo(o){let r=document.querySelector(".error-modal-overlay");if(r)r.remove();let d=document.createElement("div");d.className="error-modal-overlay modal-overlay",d.innerHTML=`
    <div class="error-modal modal">
      <div class="error-modal-header">
        <h2>\uD83D\uDEA8 Error Details</h2>
        <button class="close-button" onclick="this.closest('.error-modal-overlay').remove()">×</button>
      </div>
      <div class="error-modal-body">
        <div class="error-code">${Y(o.errorCode)}</div>
        <div class="error-message">${Y(o.errorMessage)}</div>
        <div class="error-diagnostics">
          <h3>\uD83D\uDD0D Diagnostics</h3>
          <div class="diagnostics-content">${o.diagnostics}</div>
        </div>
        ${o.debugInfo?`
          <div class="error-debug">
            <button class="debug-toggle" onclick="toggleErrorDebug(this)">\uD83D\uDC1B Show Debug Info</button>
            <div class="debug-content" style="display: none;">
              <pre>${Y(o.debugInfo)}</pre>
            </div>
          </div>
        `:""}
      </div>
      <div class="error-modal-footer">
        ${o.fullErrorPageUrl?`
          <button class="button button-primary" onclick="window.open('${o.fullErrorPageUrl}', '_blank')">
            \uD83D\uDD0D View Full Error Page
          </button>
        `:""}
        ${E()?`
          <button class="button button-warning" onclick="navigateToErrorPage('${o.originalUrl||""}')">
            \uD83D\uDEA7 Debug Mode: Go to Error Page
          </button>
        `:""}
        <button class="button button-secondary" onclick="this.closest('.error-modal-overlay').remove()">Close</button>
      </div>
    </div>
  `,document.body.appendChild(d),window.toggleErrorDebug=(x)=>{let b=x.nextElementSibling;if(b.style.display==="none")b.style.display="block",x.textContent="\uD83D\uDC1B Hide Debug Info";else b.style.display="none",x.textContent="\uD83D\uDC1B Show Debug Info"}}function $o(o){let r=Zo(),d=document.createElement("div");d.className="message message-error enhanced-error",d.innerHTML=`
    <div class="error-summary">
      <strong>${Y(o.errorCode)}</strong>
      <p>${Y(o.errorMessage)}</p>
      <button class="error-details-button" onclick="showErrorDetailsModal(this)">View Details</button>
    </div>
  `,d.errorDetails=o,r.appendChild(d),setTimeout(()=>{if(d.parentNode)d.parentNode.removeChild(d)},1e4)}function Qo(o){try{let d=new DOMParser().parseFromString(o,"text/html"),x=d.querySelector(".error-code"),b=d.querySelector(".error-message"),a=d.querySelector(".error-details"),n=d.querySelector(".debug-output");if(!x||!b)return null;return{errorCode:x.textContent?.trim()||"UNKNOWN_ERROR",errorMessage:b.textContent?.trim()||"An unknown error occurred",diagnostics:a?.innerHTML||"No diagnostic information available",debugInfo:n?.textContent||void 0}}catch(r){return console.error("Failed to parse error response:",r),null}}function Zo(){let o=document.getElementById("messages-container");if(!o){o=document.createElement("div"),o.id="messages-container",o.className="messages";let r=document.querySelector("main");if(r&&r.parentNode)r.parentNode.insertBefore(o,r);else document.body.appendChild(o)}return o}function Y(o){let r=document.createElement("div");return r.textContent=o,r.innerHTML}function E(){return!!(new URLSearchParams(window.location.search).has("debug")||localStorage.getItem("nexsock-debug")==="true"||window.location.hostname==="localhost"||window.location.hostname==="127.0.0.1"||window.NEXSOCK_DEBUG===!0)}function i(o){let r=new URL(o,window.location.origin);return r.searchParams.set("debug-error","true"),r.toString()}function D(o){if(o){let r=i(o);window.location.href=r}else l("No original URL available for error page navigation","warning")}function C(o,r){if(E()){if(localStorage.getItem("nexsock-debug-auto-redirect")==="true"&&r){console.log("Debug mode: Auto-redirecting to error page"),D(r);return}}_o(o,r)}function _o(o,r){if(o.responseText){let b=Qo(o.responseText);if(b){b.originalUrl=r,b.fullErrorPageUrl=r?i(r):void 0,$o(b);return}}let d="An error occurred while processing your request",x="error";switch(o.status){case 400:d="Bad request - please check your input and try again";break;case 401:d="Authentication required - please log in";break;case 403:d="Access denied - you don't have permission for this action";break;case 404:d="The requested resource was not found";break;case 429:d="Too many requests - please wait a moment and try again",x="warning";break;case 500:d="Internal server error - please try again later";break;case 502:case 503:case 504:d="Service temporarily unavailable - please try again later";break;default:if(o.status>=400)d=`Request failed with status ${o.status}`}l(d,x)}window.showErrorDetailsModal=(o)=>{let r=o.closest(".enhanced-error");if(r&&r.errorDetails)Yo(r.errorDetails)};window.navigateToErrorPage=D;function s(){S(),t(),document.body.addEventListener("htmx:responseError",(o)=>{let r=o;console.error("HTMX Error:",r.detail);let d=r.detail.xhr,x=r.detail.requestConfig?.path;if(d)C(d,x);else l("An error occurred while loading content","error")}),document.body.addEventListener("htmx:pushedIntoHistory",(o)=>{let r=X();if(r){let d=r.getCurrentTheme();document.documentElement.setAttribute("data-theme",d)}}),document.body.addEventListener("htmx:beforeRequest",(o)=>{let r=o.target;if(r.classList.contains("button"))r.classList.add("button-loading")}),document.body.addEventListener("htmx:afterRequest",(o)=>{let r=o.target;if(r.classList.contains("button"))r.classList.remove("button-loading");t()}),document.body.addEventListener("htmx:afterSettle",(o)=>{let r=X();if(r){let d=r.getCurrentTheme();document.documentElement.setAttribute("data-theme",d)}}),document.addEventListener("click",(o)=>{let r=o.target;if(r.classList.contains("modal-overlay"))z();if(!r.closest(".dropdown"))w()}),document.addEventListener("keydown",(o)=>{if(o.key==="Escape")z(),w()})}function m(o,r,d,x=""){let b=k.SERVICE_CONFIG(o),a=$(o);a[r]={envVars:d,description:x,lastUsed:new Date().toISOString(),created:a[r]?.created||new Date().toISOString()},localStorage.setItem(b,JSON.stringify(a)),console.log(`Saved configuration '${r}' for service '${o}'`)}function $(o){let r=k.SERVICE_CONFIG(o),d=localStorage.getItem(r);return d?JSON.parse(d):{}}function _(o,r){return $(o)[r]||null}function N(o,r){let d=k.SERVICE_CONFIG(o),x=$(o);if(x[r])return delete x[r],localStorage.setItem(d,JSON.stringify(x)),console.log(`Deleted configuration '${r}' for service '${o}'`),!0;return!1}function e(o){let r={},d=document.getElementById(`env-vars-${o}`);if(d)d.querySelectorAll(".env-var-pair").forEach((x)=>{let b=x.querySelectorAll("input"),[a,n]=b;if(a?.value)r[a.value]=n?.value||""});return r}function j(o,r){let d=document.getElementById(`env-vars-${o}`);if(!d)return;d.innerHTML="",Object.entries(r).forEach(([x,b])=>{window.htmx.ajax("GET",`/api/templates/env-var-pair?key=${encodeURIComponent(x)}&value=${encodeURIComponent(b)}`,{target:`#env-vars-${o}`,swap:"beforeend"})}),window.htmx.ajax("GET","/api/templates/env-var-pair",{target:`#env-vars-${o}`,swap:"beforeend"})}function oo(o){let r=document.getElementById(`env-vars-${o}`);if(!r)return;if(confirm("Clear all current environment variables?"))r.innerHTML="",window.htmx.ajax("GET","/api/templates/env-var-pair",{target:`#env-vars-${o}`,swap:"beforeend"}),l("Environment variables cleared","info")}function ro(o){let r=document.getElementById(`management-${o}`);if(r){let d=r.style.display==="none";r.style.display=d?"block":"none"}}function xo(o,r){if(!r)return;let d=_(o,r);if(d)j(o,d.envVars),console.log(`Loaded configuration '${r}' for service '${o}'`)}function bo(o){let r=window.nexsock.getCurrentEnvVars(o);if(Object.keys(r).length===0){l("Please add some environment variables before saving a configuration.","warning");return}let d=prompt("Enter a name for this configuration:");if(!d)return;let x=prompt("Enter a description (optional):")||"";window.nexsock.saveServiceConfig(o,d,r,x),window.nexsock.refreshConfigUI(o),l(`Configuration '${d}' saved successfully!`,"success")}function ao(o){window.htmx.ajax("GET",`/api/templates/config-section?service=${encodeURIComponent(o)}`,{target:`#config-section-${o}`,swap:"innerHTML"})}function no(o,r){if(confirm(`Are you sure you want to delete the configuration '${r}'?`))window.nexsock.deleteServiceConfig(o,r),window.htmx.ajax("GET",`/api/templates/config-modal-content?service=${encodeURIComponent(o)}`,{target:".modal-body",swap:"innerHTML"}),window.nexsock.refreshConfigUI(o),l(`Configuration '${r}' deleted successfully.`,"success")}async function fo(o){if(!o){l("Invalid service name","error");return}if(confirm(`Are you sure you want to remove ${o}? This action cannot be undone.`))try{let r=await fetch(`/api/services/${o}`,{method:"DELETE"});if(!r.ok)throw new Error(`HTTP error: ${r.status}`);l(`Service '${o}' removed successfully.`,"success"),window.location.href="/"}catch(r){console.error("Error removing service:",r),l("Failed to remove service","error")}}var lo={enabled:!1,autoRedirectToErrorPage:!1,verboseLogging:!1};function Q(){try{let o=localStorage.getItem("nexsock-debug-config");if(o)return{...lo,...JSON.parse(o)}}catch(o){console.warn("Failed to parse debug config from localStorage:",o)}return lo}function O(o){let d={...Q(),...o};try{localStorage.setItem("nexsock-debug-config",JSON.stringify(d)),localStorage.setItem("nexsock-debug",d.enabled.toString()),localStorage.setItem("nexsock-debug-auto-redirect",d.autoRedirectToErrorPage.toString()),console.log("Debug config updated:",d)}catch(x){console.error("Failed to save debug config:",x)}}function jo(o={}){O({enabled:!0,...o}),console.log("\uD83D\uDEA7 Debug mode enabled for Nexsock"),console.log("To disable: nexsock.debug.disable()"),console.log("To configure: nexsock.debug.configure({ autoRedirectToErrorPage: true })")}function Jo(){O({enabled:!1,autoRedirectToErrorPage:!1,verboseLogging:!1}),console.log("Debug mode disabled for Nexsock")}function Wo(o){let r=Q();if(!r.enabled){console.warn("Debug mode is not enabled. Enable it first with nexsock.debug.enable()");return}O(o),console.log("Debug configuration updated:",{...r,...o})}function Xo(o,...r){let d=Q();if(d.enabled&&d.verboseLogging)console.log(`[Nexsock Debug] ${o}`,...r)}function Oo(){let o=Q();if(console.group("\uD83D\uDEA7 Nexsock Debug Status"),console.log("Enabled:",o.enabled),console.log("Auto-redirect to error page:",o.autoRedirectToErrorPage),console.log("Verbose logging:",o.verboseLogging),console.groupEnd(),o.enabled)console.log(`
Available debug commands:`),console.log("- nexsock.debug.disable() - Disable debug mode"),console.log("- nexsock.debug.configure({ autoRedirectToErrorPage: true }) - Auto-redirect to error pages"),console.log("- nexsock.debug.configure({ verboseLogging: true }) - Enable verbose logging"),console.log("- nexsock.debug.testError() - Trigger a test error to see error handling");else console.log("Enable debug mode with: nexsock.debug.enable()")}function Po(){console.log("Triggering test error..."),fetch("/api/test-query-error").then((o)=>{if(!o.ok)console.log("Test error response received:",o.status)}).catch((o)=>{console.error("Test error triggered:",o)})}var po={enable:jo,disable:Jo,configure:Wo,status:Oo,testError:Po,getConfig:Q,log:Xo};var uo=()=>{return{saveServiceConfig:m,getServiceConfigs:$,loadServiceConfig:_,deleteServiceConfig:N,getCurrentEnvVars:e,applyEnvVarsToForm:j,loadConfigFromSelection:xo,showSaveConfigModal:bo,refreshConfigUI:ao,deleteConfigAndRefresh:no,toggleManagement:ro,closeModal:z,showMessage:l,confirmRemove:fo,showGitTab:U,createNewBranch:v,refreshGitSection:M,toggleDropdown:h,closeAllDropdowns:w,clearCurrentEnvVars:oo,toggleGitContent:T,restoreGitContentVisibility:t,debug:po}};document.addEventListener("DOMContentLoaded",()=>{s(),window.nexsock=uo(),G(),console.log("Nexsock web interface initialized successfully")});})();

//# debugId=E30CB613C7AC268764756E2164756E21
//# sourceMappingURL=main.js.map
