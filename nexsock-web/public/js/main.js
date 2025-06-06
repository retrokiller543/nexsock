(()=>{var P=null;function J(r){return`nx-${r}-${Math.random().toString(36).substring(2,8)}`}function L(r,b){if(!r||!b)return;let o=r.replace(/([^{}]+){/g,(d,f)=>{let u=f.trim();if(u.startsWith("@")||u.includes(`[data-scope="${b}"]`))return d;return`${u.split(",").map((q)=>{let $=q.trim();return`[data-scope="${b}"] ${$}`}).join(", ")} {`}),x=document.createElement("style");x.textContent=o,x.setAttribute("data-scope-id",b),document.head.appendChild(x)}function or(r,b,...o){if(typeof r==="function"){if(r&&typeof r==="object"&&"component"in r&&"css"in r){let d=r,f=d.component.name||"Component",u=J(f);L(d.css,u);let z=P;P=u;let q=d.component({...b,children:o});return P=z,q}if(r.css){let d=r.name||"Component",f=J(d);L(r.css,f);let u=P;P=f;let z=r({...b,children:o});return P=u,z}return r({...b,children:o})}let x=document.createElement(r);if(P)x.setAttribute("data-scope",P);if(b)Object.entries(b).forEach(([d,f])=>{if(d==="className")x.className=f;else if(d==="css"){let u=J("inline");L(f,u),x.setAttribute("data-scope",u)}else if(d.startsWith("on")&&typeof f==="function"){let u=d.toLowerCase().slice(2);x.addEventListener(u,f)}else x.setAttribute(d,f)});return o.flat().forEach((d)=>{if(typeof d==="string"||typeof d==="number")x.appendChild(document.createTextNode(String(d)));else if(d instanceof Node)x.appendChild(d)}),x}function xr({children:r}){let b=document.createDocumentFragment();return r.flat().forEach((o)=>{if(typeof o==="string"||typeof o==="number")b.appendChild(document.createTextNode(String(o)));else if(o instanceof Node)b.appendChild(o)}),b}globalThis.createElement=or;globalThis.Fragment=xr;var F=`.ns-card {
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
}`;function H(r){let{children:b,title:o,subtitle:x,footer:d,variant:f="default",className:u=""}=r,z=["ns-card",f!=="default"&&f,u].filter(Boolean).join(" ");return createElement("div",{className:z},(o||x)&&createElement("div",{className:"ns-card-header"},o&&createElement("h3",{className:"ns-card-title"},o),x&&createElement("p",{className:"ns-card-subtitle"},x)),b&&createElement("div",{className:"ns-card-body"},b),d&&createElement("div",{className:"ns-card-footer"},d))}H.css=F;var K=H;var B=`.ns-badge {
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
}`;function R(r){let{children:b,variant:o="neutral",size:x="medium",style:d="default",className:f=""}=r,u=["ns-badge",o,x!=="medium"&&x,d!=="default"&&d,f].filter(Boolean).join(" ");return createElement("span",{className:u},b)}R.css=B;var A=R;var G=`.ns-button {
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
}`;function V(r){let{children:b,variant:o="primary",size:x="medium",disabled:d=!1,loading:f=!1,type:u="button",onClick:z,href:q,target:$,className:br=""}=r,j=["ns-button",o,x,f&&"loading",br].filter(Boolean).join(" ");if(q)return createElement("a",{href:q,target:$,className:j,onClick:z,"aria-disabled":d},b);return createElement("button",{type:u,className:j,...d?{disabled:!0}:{},onClick:z},b)}V.css=G;var D=V;function dr(r,b){if(b.css&&!document.querySelector(`style[data-component="${r}"]`)){let x=document.createElement("style");x.textContent=b.css,x.setAttribute("data-component",r),document.head.appendChild(x)}class o extends HTMLElement{mounted=!1;static get observedAttributes(){return b.observedAttributes||[]}connectedCallback(){this.render(),this.mounted=!0}attributeChangedCallback(){if(this.mounted)this.render()}render(){let x={};for(let z=0;z<this.attributes.length;z++){let q=this.attributes[z];if(!q)continue;let $=q.value;if($.startsWith("{")||$.startsWith("[")||$==="true"||$==="false")try{$=JSON.parse($)}catch{}x[this.camelCase(q.name)]=$}for(let z in x)if(z.startsWith("on")&&typeof x[z]==="string"){let q=z.slice(2).toLowerCase(),$=x[z];if($.includes("alert")||$.includes("console"))x[z]=new Function("event",$)}let d=this.innerHTML,f=[];if(d.trim()){let z=document.createElement("div");z.innerHTML=d,f.push(...Array.from(z.childNodes))}if(f.length>0)x.children=f;this.innerHTML="";let u=b.component(x);if(u instanceof Node)this.appendChild(u)}camelCase(x){return x.replace(/-([a-z])/g,(d,f)=>f.toUpperCase())}}customElements.define(r,o)}function l(r,b){dr(r,{component:b,css:b.css,observedAttributes:["*"]})}var y={"ns-card":K,"ns-badge":A,"ns-button":D};function M(){Object.entries(y).forEach(([r,b])=>{l(r,b)}),console.log("Auto-registered components:",Object.keys(y))}function _(r,b="info"){let o=document.createElement("div");o.className=`message message-${b}`,o.textContent=r;let x=document.getElementById("messages-container");if(!x)x=document.createElement("div"),x.id="messages-container",x.className="messages",document.body.appendChild(x);x.appendChild(o),setTimeout(()=>{if(o.parentNode)o.parentNode.removeChild(o)},5000)}function Q(){let r=document.querySelector(".modal-overlay");if(r)r.remove()}function E(r){let b=document.getElementById(r);if(!b)return;document.querySelectorAll(".dropdown.active").forEach((o)=>{if(o.id!==r)o.classList.remove("active")}),b.classList.toggle("active")}function Z(){document.querySelectorAll(".dropdown.active").forEach((r)=>{r.classList.remove("active")})}var O={SERVICE_CONFIG:(r)=>`nexsock_service_config_${r}`,GIT_CONTENT_COLLAPSED:(r)=>`git_${r}_collapsed`};function T(r,b){document.querySelectorAll(".tab-button").forEach((d)=>{d.classList.remove("active")});let o=event?.target;if(o)o.classList.add("active");let x=document.getElementById("git-tab-content");if(!x)return;if(r==="commits")x.innerHTML='<div class="loading">Loading commits...</div>',window.htmx.ajax("GET",`/api/templates/git-log?service=${b}`,{target:"#git-tab-content",swap:"innerHTML"});else if(r==="branches")x.innerHTML='<div class="loading">Loading branches...</div>',window.htmx.ajax("GET",`/api/templates/git-branches?service=${b}`,{target:"#git-tab-content",swap:"innerHTML"})}function p(r){let b=document.getElementById("new-branch-name");if(!b)return;let o=b.value.trim();if(!o){_("Please enter a branch name","warning");return}if(!confirm(`Create new branch "${o}" and switch to it?`))return;let x=new FormData;x.append("branch",o),x.append("create","true"),fetch(`/api/services/${r}/git/checkout/branch`,{method:"POST",body:x}).then((d)=>{if(!d.ok)throw new Error(`HTTP error: ${d.status}`);return d.json()}).then((d)=>{b.value="",window.htmx.ajax("GET",`/api/templates/git-section?service=${r}`,{target:"#git-section",swap:"outerHTML"}),_(`Successfully created and switched to branch "${o}"`,"success")}).catch((d)=>{console.error("Error creating branch:",d),_("Failed to create branch","error")})}function C(r){window.htmx.ajax("GET",`/api/templates/git-section?service=${r}`,{target:"#git-section",swap:"outerHTML"})}function I(r){let b=document.getElementById(r);if(!b)return;b.classList.toggle("collapsed");let o=b.classList.contains("collapsed");localStorage.setItem(O.GIT_CONTENT_COLLAPSED(r),o.toString())}function k(){let r=localStorage.getItem(O.GIT_CONTENT_COLLAPSED("git-commits-list"))==="true",b=document.getElementById("git-commits-list");if(b&&r)b.classList.add("collapsed");let o=localStorage.getItem(O.GIT_CONTENT_COLLAPSED("git-branches-list"))==="true",x=document.getElementById("git-branches-list");if(x&&o)x.classList.add("collapsed")}var fr={light:"☀️",dark:"\uD83C\uDF19",auto:"\uD83D\uDCBB"};class g{currentTheme="auto";isInitialized=!1;constructor(){if(!this.isInitialized)this.initializeTheme(),this.setupThemeToggle(),this.isInitialized=!0}initializeTheme(){let r=localStorage.getItem("nexsock-theme");if(r&&this.isValidTheme(r))this.currentTheme=r;else this.currentTheme="auto";this.applyTheme(),this.updateThemeIcon()}setupThemeToggle(){let r=document.getElementById("theme-toggle");if(r&&!r.hasAttribute("data-theme-listener")){let b=()=>{this.toggleTheme()};r.addEventListener("click",b),r.setAttribute("data-theme-listener","true")}}toggleTheme(){let r=["light","dark","auto"],o=(r.indexOf(this.currentTheme)+1)%r.length,x=r[o];if(x)this.setTheme(x)}setTheme(r){if(!this.isValidTheme(r)){console.warn(`Invalid theme: ${r}`);return}this.currentTheme=r,localStorage.setItem("nexsock-theme",r),this.applyTheme(),this.updateThemeIcon()}applyTheme(){document.documentElement.setAttribute("data-theme",this.currentTheme)}updateThemeIcon(){let r=document.querySelector(".theme-icon");if(r)r.textContent=fr[this.currentTheme];let b=document.getElementById("theme-toggle");if(b)b.setAttribute("aria-label",`Current theme: ${this.currentTheme}. Click to change theme.`)}isValidTheme(r){return["light","dark","auto"].includes(r)}getCurrentTheme(){return this.currentTheme}getEffectiveTheme(){if(this.currentTheme==="auto")return window.matchMedia("(prefers-color-scheme: dark)").matches?"dark":"light";return this.currentTheme}}var Y=null;function S(){if(!Y)Y=new g;return Y}function X(){return Y}function a(){S(),k(),document.body.addEventListener("htmx:responseError",(r)=>{console.error("HTMX Error:",r.detail),_("An error occurred while loading content","error")}),document.body.addEventListener("htmx:pushedIntoHistory",(r)=>{let b=X();if(b){let o=b.getCurrentTheme();document.documentElement.setAttribute("data-theme",o)}}),document.body.addEventListener("htmx:beforeRequest",(r)=>{let b=r.target;if(b.classList.contains("button"))b.classList.add("button-loading")}),document.body.addEventListener("htmx:afterRequest",(r)=>{let b=r.target;if(b.classList.contains("button"))b.classList.remove("button-loading");k()}),document.body.addEventListener("htmx:afterSettle",(r)=>{let b=X();if(b){let o=b.getCurrentTheme();document.documentElement.setAttribute("data-theme",o)}}),document.addEventListener("click",(r)=>{let b=r.target;if(b.classList.contains("modal-overlay"))Q();if(!b.closest(".dropdown"))Z()}),document.addEventListener("keydown",(r)=>{if(r.key==="Escape")Q(),Z()})}function n(r,b,o,x=""){let d=O.SERVICE_CONFIG(r),f=U(r);f[b]={envVars:o,description:x,lastUsed:new Date().toISOString(),created:f[b]?.created||new Date().toISOString()},localStorage.setItem(d,JSON.stringify(f)),console.log(`Saved configuration '${b}' for service '${r}'`)}function U(r){let b=O.SERVICE_CONFIG(r),o=localStorage.getItem(b);return o?JSON.parse(o):{}}function W(r,b){return U(r)[b]||null}function h(r,b){let o=O.SERVICE_CONFIG(r),x=U(r);if(x[b])return delete x[b],localStorage.setItem(o,JSON.stringify(x)),console.log(`Deleted configuration '${b}' for service '${r}'`),!0;return!1}function v(r){let b={},o=document.getElementById(`env-vars-${r}`);if(o)o.querySelectorAll(".env-var-pair").forEach((x)=>{let d=x.querySelectorAll("input"),[f,u]=d;if(f?.value)b[f.value]=u?.value||""});return b}function w(r,b){let o=document.getElementById(`env-vars-${r}`);if(!o)return;o.innerHTML="",Object.entries(b).forEach(([x,d])=>{window.htmx.ajax("GET",`/api/templates/env-var-pair?key=${encodeURIComponent(x)}&value=${encodeURIComponent(d)}`,{target:`#env-vars-${r}`,swap:"beforeend"})}),window.htmx.ajax("GET","/api/templates/env-var-pair",{target:`#env-vars-${r}`,swap:"beforeend"})}function t(r){let b=document.getElementById(`env-vars-${r}`);if(!b)return;if(confirm("Clear all current environment variables?"))b.innerHTML="",window.htmx.ajax("GET","/api/templates/env-var-pair",{target:`#env-vars-${r}`,swap:"beforeend"}),_("Environment variables cleared","info")}function N(r){let b=document.getElementById(`management-${r}`);if(b){let o=b.style.display==="none";b.style.display=o?"block":"none"}}function c(r,b){if(!b)return;let o=W(r,b);if(o)w(r,o.envVars),console.log(`Loaded configuration '${b}' for service '${r}'`)}function s(r){let b=window.nexsock.getCurrentEnvVars(r);if(Object.keys(b).length===0){_("Please add some environment variables before saving a configuration.","warning");return}let o=prompt("Enter a name for this configuration:");if(!o)return;let x=prompt("Enter a description (optional):")||"";window.nexsock.saveServiceConfig(r,o,b,x),window.nexsock.refreshConfigUI(r),_(`Configuration '${o}' saved successfully!`,"success")}function i(r){window.htmx.ajax("GET",`/api/templates/config-section?service=${encodeURIComponent(r)}`,{target:`#config-section-${r}`,swap:"innerHTML"})}function m(r,b){if(confirm(`Are you sure you want to delete the configuration '${b}'?`))window.nexsock.deleteServiceConfig(r,b),window.htmx.ajax("GET",`/api/templates/config-modal-content?service=${encodeURIComponent(r)}`,{target:".modal-body",swap:"innerHTML"}),window.nexsock.refreshConfigUI(r),_(`Configuration '${b}' deleted successfully.`,"success")}async function e(r){if(!r){_("Invalid service name","error");return}if(confirm(`Are you sure you want to remove ${r}? This action cannot be undone.`))try{let b=await fetch(`/api/services/${r}`,{method:"DELETE"});if(!b.ok)throw new Error(`HTTP error: ${b.status}`);_(`Service '${r}' removed successfully.`,"success"),window.location.href="/"}catch(b){console.error("Error removing service:",b),_("Failed to remove service","error")}}var rr=()=>{return{saveServiceConfig:n,getServiceConfigs:U,loadServiceConfig:W,deleteServiceConfig:h,getCurrentEnvVars:v,applyEnvVarsToForm:w,loadConfigFromSelection:c,showSaveConfigModal:s,refreshConfigUI:i,deleteConfigAndRefresh:m,toggleManagement:N,closeModal:Q,showMessage:_,confirmRemove:e,showGitTab:T,createNewBranch:p,refreshGitSection:C,toggleDropdown:E,closeAllDropdowns:Z,clearCurrentEnvVars:t,toggleGitContent:I,restoreGitContentVisibility:k}};document.addEventListener("DOMContentLoaded",()=>{a(),window.nexsock=rr(),M(),console.log("Nexsock web interface initialized successfully")});})();

//# debugId=EFDE168A87FB1C1E64756E2164756E21
//# sourceMappingURL=main.js.map
