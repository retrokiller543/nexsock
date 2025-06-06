(()=>{var Q=null;function Y(b){return`nx-${b}-${Math.random().toString(36).substring(2,8)}`}function F(b,x){if(!b||!x)return;let d=b.replace(/([^{}]+){/g,(o,u)=>{let f=u.trim();if(f.startsWith("@")||f.includes(`[data-scope="${x}"]`))return o;return`${f.split(",").map((q)=>{let $=q.trim();return`[data-scope="${x}"] ${$}`}).join(", ")} {`}),r=document.createElement("style");r.textContent=d,r.setAttribute("data-scope-id",x),document.head.appendChild(r)}function m(b,x,...d){if(typeof b==="function"){if(b&&typeof b==="object"&&"component"in b&&"css"in b){let o=b,u=o.component.name||"Component",f=Y(u);F(o.css,f);let z=Q;Q=f;let q=o.component({...x,children:d});return Q=z,q}if(b.css){let o=b.name||"Component",u=Y(o);F(b.css,u);let f=Q;Q=u;let z=b({...x,children:d});return Q=f,z}return b({...x,children:d})}let r=document.createElement(b);if(Q)r.setAttribute("data-scope",Q);if(x)Object.entries(x).forEach(([o,u])=>{if(o==="className")r.className=u;else if(o==="css"){let f=Y("inline");F(u,f),r.setAttribute("data-scope",f)}else if(o.startsWith("on")&&typeof u==="function"){let f=o.toLowerCase().slice(2);r.addEventListener(f,u)}else r.setAttribute(o,u)});return d.flat().forEach((o)=>{if(typeof o==="string"||typeof o==="number")r.appendChild(document.createTextNode(String(o)));else if(o instanceof Node)r.appendChild(o)}),r}function s({children:b}){let x=document.createDocumentFragment();return b.flat().forEach((d)=>{if(typeof d==="string"||typeof d==="number")x.appendChild(document.createTextNode(String(d)));else if(d instanceof Node)x.appendChild(d)}),x}globalThis.createElement=m;globalThis.Fragment=s;function e(b,x){class d extends HTMLElement{mounted=!1;static get observedAttributes(){return x.observedAttributes||[]}connectedCallback(){this.render(),this.mounted=!0}attributeChangedCallback(){if(this.mounted)this.render()}render(){let r={};for(let z=0;z<this.attributes.length;z++){let q=this.attributes[z];if(!q)continue;let $=q.value;if($.startsWith("{")||$.startsWith("[")||$==="true"||$==="false")try{$=JSON.parse($)}catch{}r[this.camelCase(q.name)]=$}for(let z in r)if(z.startsWith("on")&&typeof r[z]==="string"){let q=z.slice(2).toLowerCase(),$=r[z];if($.includes("alert")||$.includes("console"))r[z]=new Function("event",$)}let o=this.innerHTML,u=[];if(o.trim()){let z=document.createElement("div");z.innerHTML=o,u.push(...Array.from(z.childNodes))}if(u.length>0)r.children=u;this.innerHTML="";let f=x.component(r);if(f instanceof Node)this.appendChild(f)}camelCase(r){return r.replace(/-([a-z])/g,(o,u)=>u.toUpperCase())}}customElements.define(b,d)}function H(b,x){e(b,{component:x,css:x.css,observedAttributes:["*"]})}var O=`.ns-button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  padding: 8px 16px;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-weight: 500;
  font-size: 14px;
  line-height: 1.4;
  text-decoration: none;
  transition: all 0.15s ease;
  user-select: none;
  min-height: 36px;
  gap: 6px;
}

.ns-button:focus {
  outline: 2px solid var(--focus-color, #3b82f6);
  outline-offset: 2px;
}

.ns-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
  pointer-events: none;
}

/* Variants */
.ns-button.primary {
  background-color: var(--primary-color, #3b82f6);
  color: white;
}

.ns-button.primary:hover:not(:disabled) {
  background-color: var(--primary-hover, #2563eb);
  transform: translateY(-1px);
}

.ns-button.secondary {
  background-color: var(--secondary-bg, #f3f4f6);
  color: var(--secondary-color, #374151);
  border: 1px solid var(--secondary-border, #d1d5db);
}

.ns-button.secondary:hover:not(:disabled) {
  background-color: var(--secondary-hover, #e5e7eb);
  border-color: var(--secondary-border-hover, #9ca3af);
}

.ns-button.danger {
  background-color: var(--danger-color, #ef4444);
  color: white;
}

.ns-button.danger:hover:not(:disabled) {
  background-color: var(--danger-hover, #dc2626);
  transform: translateY(-1px);
}

.ns-button.ghost {
  background-color: transparent;
  color: var(--ghost-color, #6b7280);
  border: 1px solid transparent;
}

.ns-button.ghost:hover:not(:disabled) {
  background-color: var(--ghost-hover, #f9fafb);
  color: var(--ghost-color-hover, #374151);
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
}`;function j(b){let{children:x,variant:d="primary",size:r="medium",disabled:o=!1,loading:u=!1,type:f="button",onClick:z,href:q,target:$,className:i=""}=b,w=["ns-button",d,r,u&&"loading",i].filter(Boolean).join(" ");if(q)return createElement("a",{href:q,target:$,className:w,onClick:z,"aria-disabled":o},x);return createElement("button",{type:f,className:w,...o?{disabled:!0}:{},onClick:z},x)}j.css=O;var K=j;var T=`.ns-card {
  background: var(--card-bg, #ffffff);
  border: 1px solid var(--card-border, #e5e7eb);
  border-radius: var(--card-radius, 8px);
  box-shadow: var(--card-shadow, 0 1px 3px rgba(0, 0, 0, 0.1));
  overflow: hidden;
  transition: box-shadow 0.15s ease;
}

.ns-card:hover {
  box-shadow: var(--card-shadow-hover, 0 4px 12px rgba(0, 0, 0, 0.15));
}

.ns-card-header {
  padding: 16px 20px;
  border-bottom: 1px solid var(--card-border, #e5e7eb);
  background: var(--card-header-bg, #f9fafb);
}

.ns-card-title {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: var(--card-title-color, #111827);
}

.ns-card-subtitle {
  margin: 4px 0 0 0;
  font-size: 14px;
  color: var(--card-subtitle-color, #6b7280);
}

.ns-card-body {
  padding: 20px;
}

.ns-card-footer {
  padding: 12px 20px;
  border-top: 1px solid var(--card-border, #e5e7eb);
  background: var(--card-footer-bg, #f9fafb);
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

/* Variants */
.ns-card.elevated {
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.12);
}

.ns-card.flat {
  box-shadow: none;
  border: 1px solid var(--card-border, #e5e7eb);
}

.ns-card.borderless {
  border: none;
  box-shadow: none;
}`;function G(b){let{children:x,title:d,subtitle:r,footer:o,variant:u="default",className:f=""}=b,z=["ns-card",u!=="default"&&u,f].filter(Boolean).join(" ");return createElement("div",{className:z},(d||r)&&createElement("div",{className:"ns-card-header"},d&&createElement("h3",{className:"ns-card-title"},d),r&&createElement("p",{className:"ns-card-subtitle"},r)),x&&createElement("div",{className:"ns-card-body"},x),o&&createElement("div",{className:"ns-card-footer"},o))}G.css=T;var R=G;var A=`.ns-badge {
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
}`;function V(b){let{children:x,variant:d="neutral",size:r="medium",style:o="default",className:u=""}=b,f=["ns-badge",d,r!=="medium"&&r,o!=="default"&&o,u].filter(Boolean).join(" ");return createElement("span",{className:f},x)}V.css=A;var D=V;function B(){H("ns-button",K),H("ns-card",R),H("ns-badge",D),console.log("Nexsock UI components registered:",["ns-button","ns-card","ns-badge"])}function _(b,x="info"){let d=document.createElement("div");d.className=`message message-${x}`,d.textContent=b;let r=document.getElementById("messages-container");if(!r)r=document.createElement("div"),r.id="messages-container",r.className="messages",document.body.appendChild(r);r.appendChild(d),setTimeout(()=>{if(d.parentNode)d.parentNode.removeChild(d)},5000)}function P(){let b=document.querySelector(".modal-overlay");if(b)b.remove()}function M(b){let x=document.getElementById(b);if(!x)return;document.querySelectorAll(".dropdown.active").forEach((d)=>{if(d.id!==b)d.classList.remove("active")}),x.classList.toggle("active")}function k(){document.querySelectorAll(".dropdown.active").forEach((b)=>{b.classList.remove("active")})}var Z={SERVICE_CONFIG:(b)=>`nexsock_service_config_${b}`,GIT_CONTENT_COLLAPSED:(b)=>`git_${b}_collapsed`};function h(b,x){document.querySelectorAll(".tab-button").forEach((o)=>{o.classList.remove("active")});let d=event?.target;if(d)d.classList.add("active");let r=document.getElementById("git-tab-content");if(!r)return;if(b==="commits")r.innerHTML='<div class="loading">Loading commits...</div>',window.htmx.ajax("GET",`/api/templates/git-log?service=${x}`,{target:"#git-tab-content",swap:"innerHTML"});else if(b==="branches")r.innerHTML='<div class="loading">Loading branches...</div>',window.htmx.ajax("GET",`/api/templates/git-branches?service=${x}`,{target:"#git-tab-content",swap:"innerHTML"})}function E(b){let x=document.getElementById("new-branch-name");if(!x)return;let d=x.value.trim();if(!d){_("Please enter a branch name","warning");return}if(!confirm(`Create new branch "${d}" and switch to it?`))return;let r=new FormData;r.append("branch",d),r.append("create","true"),fetch(`/api/services/${b}/git/checkout/branch`,{method:"POST",body:r}).then((o)=>{if(!o.ok)throw new Error(`HTTP error: ${o.status}`);return o.json()}).then((o)=>{x.value="",window.htmx.ajax("GET",`/api/templates/git-section?service=${b}`,{target:"#git-section",swap:"outerHTML"}),_(`Successfully created and switched to branch "${d}"`,"success")}).catch((o)=>{console.error("Error creating branch:",o),_("Failed to create branch","error")})}function y(b){window.htmx.ajax("GET",`/api/templates/git-section?service=${b}`,{target:"#git-section",swap:"outerHTML"})}function l(b){let x=document.getElementById(b);if(!x)return;x.classList.toggle("collapsed");let d=x.classList.contains("collapsed");localStorage.setItem(Z.GIT_CONTENT_COLLAPSED(b),d.toString())}function U(){let b=localStorage.getItem(Z.GIT_CONTENT_COLLAPSED("git-commits-list"))==="true",x=document.getElementById("git-commits-list");if(x&&b)x.classList.add("collapsed");let d=localStorage.getItem(Z.GIT_CONTENT_COLLAPSED("git-branches-list"))==="true",r=document.getElementById("git-branches-list");if(r&&d)r.classList.add("collapsed")}function I(b,x,d,r=""){let o=Z.SERVICE_CONFIG(b),u=W(b);u[x]={envVars:d,description:r,lastUsed:new Date().toISOString(),created:u[x]?.created||new Date().toISOString()},localStorage.setItem(o,JSON.stringify(u)),console.log(`Saved configuration '${x}' for service '${b}'`)}function W(b){let x=Z.SERVICE_CONFIG(b),d=localStorage.getItem(x);return d?JSON.parse(d):{}}function J(b,x){return W(b)[x]||null}function S(b,x){let d=Z.SERVICE_CONFIG(b),r=W(b);if(r[x])return delete r[x],localStorage.setItem(d,JSON.stringify(r)),console.log(`Deleted configuration '${x}' for service '${b}'`),!0;return!1}function C(b){let x={},d=document.getElementById(`env-vars-${b}`);if(d)d.querySelectorAll(".env-var-pair").forEach((r)=>{let o=r.querySelectorAll("input"),[u,f]=o;if(u?.value)x[u.value]=f?.value||""});return x}function L(b,x){let d=document.getElementById(`env-vars-${b}`);if(!d)return;d.innerHTML="",Object.entries(x).forEach(([r,o])=>{window.htmx.ajax("GET",`/api/templates/env-var-pair?key=${encodeURIComponent(r)}&value=${encodeURIComponent(o)}`,{target:`#env-vars-${b}`,swap:"beforeend"})}),window.htmx.ajax("GET","/api/templates/env-var-pair",{target:`#env-vars-${b}`,swap:"beforeend"})}function g(b){let x=document.getElementById(`env-vars-${b}`);if(!x)return;if(confirm("Clear all current environment variables?"))x.innerHTML="",window.htmx.ajax("GET","/api/templates/env-var-pair",{target:`#env-vars-${b}`,swap:"beforeend"}),_("Environment variables cleared","info")}function p(b){let x=document.getElementById(`management-${b}`);if(x){let d=x.style.display==="none";x.style.display=d?"block":"none"}}function v(b,x){if(!x)return;let d=J(b,x);if(d)L(b,d.envVars),console.log(`Loaded configuration '${x}' for service '${b}'`)}function n(b){let x=window.nexsock.getCurrentEnvVars(b);if(Object.keys(x).length===0){_("Please add some environment variables before saving a configuration.","warning");return}let d=prompt("Enter a name for this configuration:");if(!d)return;let r=prompt("Enter a description (optional):")||"";window.nexsock.saveServiceConfig(b,d,x,r),window.nexsock.refreshConfigUI(b),_(`Configuration '${d}' saved successfully!`,"success")}function X(b){window.htmx.ajax("GET",`/api/templates/config-section?service=${encodeURIComponent(b)}`,{target:`#config-section-${b}`,swap:"innerHTML"})}function a(b,x){if(confirm(`Are you sure you want to delete the configuration '${x}'?`))window.nexsock.deleteServiceConfig(b,x),window.htmx.ajax("GET",`/api/templates/config-modal-content?service=${encodeURIComponent(b)}`,{target:".modal-body",swap:"innerHTML"}),window.nexsock.refreshConfigUI(b),_(`Configuration '${x}' deleted successfully.`,"success")}async function N(b){if(!b){_("Invalid service name","error");return}if(confirm(`Are you sure you want to remove ${b}? This action cannot be undone.`))try{let x=await fetch(`/api/services/${b}`,{method:"DELETE"});if(!x.ok)throw new Error(`HTTP error: ${x.status}`);_(`Service '${b}' removed successfully.`,"success"),window.location.href="/"}catch(x){console.error("Error removing service:",x),_("Failed to remove service","error")}}function t(){document.querySelectorAll("[data-service-name]").forEach((b)=>{let x=b.getAttribute("data-service-name");if(x)X(x)}),U(),document.body.addEventListener("htmx:responseError",(b)=>{console.error("HTMX Error:",b.detail),_("An error occurred while loading content","error")}),document.body.addEventListener("htmx:beforeRequest",(b)=>{let x=b.target;if(x.classList.contains("button"))x.classList.add("button-loading")}),document.body.addEventListener("htmx:afterRequest",(b)=>{let x=b.target;if(x.classList.contains("button"))x.classList.remove("button-loading");U()}),document.addEventListener("click",(b)=>{let x=b.target;if(x.classList.contains("modal-overlay"))P();if(!x.closest(".dropdown"))k()}),document.addEventListener("keydown",(b)=>{if(b.key==="Escape")P(),k()})}var c=()=>{return{saveServiceConfig:I,getServiceConfigs:W,loadServiceConfig:J,deleteServiceConfig:S,getCurrentEnvVars:C,applyEnvVarsToForm:L,loadConfigFromSelection:v,showSaveConfigModal:n,refreshConfigUI:X,deleteConfigAndRefresh:a,toggleManagement:p,closeModal:P,showMessage:_,confirmRemove:N,showGitTab:h,createNewBranch:E,refreshGitSection:y,toggleDropdown:M,closeAllDropdowns:k,clearCurrentEnvVars:g,toggleGitContent:l,restoreGitContentVisibility:U}};document.addEventListener("DOMContentLoaded",()=>{t(),window.nexsock=c(),B(),console.log("Nexsock web interface initialized successfully")});})();

//# debugId=FF78CB969B84A11364756E2164756E21
//# sourceMappingURL=main.js.map
