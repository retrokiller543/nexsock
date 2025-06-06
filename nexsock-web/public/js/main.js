(()=>{var O=null;function H(b){return`nx-${b}-${Math.random().toString(36).substring(2,8)}`}function J(b,o){if(!b||!o)return;let x=b.replace(/([^{}]+){/g,(d,u)=>{let f=u.trim();if(f.startsWith("@")||f.includes(`[data-scope="${o}"]`))return d;return`${f.split(",").map((q)=>{let $=q.trim();return`[data-scope="${o}"] ${$}`}).join(", ")} {`}),r=document.createElement("style");r.textContent=x,r.setAttribute("data-scope-id",o),document.head.appendChild(r)}function m(b,o,...x){if(typeof b==="function"){if(b&&typeof b==="object"&&"component"in b&&"css"in b){let d=b,u=d.component.name||"Component",f=H(u);J(d.css,f);let z=O;O=f;let q=d.component({...o,children:x});return O=z,q}if(b.css){let d=b.name||"Component",u=H(d);J(b.css,u);let f=O;O=u;let z=b({...o,children:x});return O=f,z}return b({...o,children:x})}let r=document.createElement(b);if(O)r.setAttribute("data-scope",O);if(o)Object.entries(o).forEach(([d,u])=>{if(d==="className")r.className=u;else if(d==="css"){let f=H("inline");J(u,f),r.setAttribute("data-scope",f)}else if(d.startsWith("on")&&typeof u==="function"){let f=d.toLowerCase().slice(2);r.addEventListener(f,u)}else r.setAttribute(d,u)});return x.flat().forEach((d)=>{if(typeof d==="string"||typeof d==="number")r.appendChild(document.createTextNode(String(d)));else if(d instanceof Node)r.appendChild(d)}),r}function e({children:b}){let o=document.createDocumentFragment();return b.flat().forEach((x)=>{if(typeof x==="string"||typeof x==="number")o.appendChild(document.createTextNode(String(x)));else if(x instanceof Node)o.appendChild(x)}),o}globalThis.createElement=m;globalThis.Fragment=e;var X=`.ns-card {
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
}`;function j(b){let{children:o,title:x,subtitle:r,footer:d,variant:u="default",className:f=""}=b,z=["ns-card",u!=="default"&&u,f].filter(Boolean).join(" ");return createElement("div",{className:z},(x||r)&&createElement("div",{className:"ns-card-header"},x&&createElement("h3",{className:"ns-card-title"},x),r&&createElement("p",{className:"ns-card-subtitle"},r)),o&&createElement("div",{className:"ns-card-body"},o),d&&createElement("div",{className:"ns-card-footer"},d))}j.css=X;var w=j;var F=`.ns-badge {
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
}`;function K(b){let{children:o,variant:x="neutral",size:r="medium",style:d="default",className:u=""}=b,f=["ns-badge",x,r!=="medium"&&r,d!=="default"&&d,u].filter(Boolean).join(" ");return createElement("span",{className:f},o)}K.css=F;var B=K;var R=`.ns-button {
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
}`;function G(b){let{children:o,variant:x="primary",size:r="medium",disabled:d=!1,loading:u=!1,type:f="button",onClick:z,href:q,target:$,className:s=""}=b,L=["ns-button",x,r,u&&"loading",s].filter(Boolean).join(" ");if(q)return createElement("a",{href:q,target:$,className:L,onClick:z,"aria-disabled":d},o);return createElement("button",{type:f,className:L,...d?{disabled:!0}:{},onClick:z},o)}G.css=R;var A=G;function bb(b,o){class x extends HTMLElement{mounted=!1;static get observedAttributes(){return o.observedAttributes||[]}connectedCallback(){this.render(),this.mounted=!0}attributeChangedCallback(){if(this.mounted)this.render()}render(){let r={};for(let z=0;z<this.attributes.length;z++){let q=this.attributes[z];if(!q)continue;let $=q.value;if($.startsWith("{")||$.startsWith("[")||$==="true"||$==="false")try{$=JSON.parse($)}catch{}r[this.camelCase(q.name)]=$}for(let z in r)if(z.startsWith("on")&&typeof r[z]==="string"){let q=z.slice(2).toLowerCase(),$=r[z];if($.includes("alert")||$.includes("console"))r[z]=new Function("event",$)}let d=this.innerHTML,u=[];if(d.trim()){let z=document.createElement("div");z.innerHTML=d,u.push(...Array.from(z.childNodes))}if(u.length>0)r.children=u;this.innerHTML="";let f=o.component(r);if(f instanceof Node)this.appendChild(f)}camelCase(r){return r.replace(/-([a-z])/g,(d,u)=>u.toUpperCase())}}customElements.define(b,x)}function V(b,o){bb(b,{component:o,css:o.css,observedAttributes:["*"]})}var D={"ns-card":w,"ns-badge":B,"ns-button":A};function M(){Object.entries(D).forEach(([b,o])=>{V(b,o)}),console.log("Auto-registered components:",Object.keys(D))}function _(b,o="info"){let x=document.createElement("div");x.className=`message message-${o}`,x.textContent=b;let r=document.getElementById("messages-container");if(!r)r=document.createElement("div"),r.id="messages-container",r.className="messages",document.body.appendChild(r);r.appendChild(x),setTimeout(()=>{if(x.parentNode)x.parentNode.removeChild(x)},5000)}function k(){let b=document.querySelector(".modal-overlay");if(b)b.remove()}function E(b){let o=document.getElementById(b);if(!o)return;document.querySelectorAll(".dropdown.active").forEach((x)=>{if(x.id!==b)x.classList.remove("active")}),o.classList.toggle("active")}function Q(){document.querySelectorAll(".dropdown.active").forEach((b)=>{b.classList.remove("active")})}var P={SERVICE_CONFIG:(b)=>`nexsock_service_config_${b}`,GIT_CONTENT_COLLAPSED:(b)=>`git_${b}_collapsed`};function h(b,o){document.querySelectorAll(".tab-button").forEach((d)=>{d.classList.remove("active")});let x=event?.target;if(x)x.classList.add("active");let r=document.getElementById("git-tab-content");if(!r)return;if(b==="commits")r.innerHTML='<div class="loading">Loading commits...</div>',window.htmx.ajax("GET",`/api/templates/git-log?service=${o}`,{target:"#git-tab-content",swap:"innerHTML"});else if(b==="branches")r.innerHTML='<div class="loading">Loading branches...</div>',window.htmx.ajax("GET",`/api/templates/git-branches?service=${o}`,{target:"#git-tab-content",swap:"innerHTML"})}function y(b){let o=document.getElementById("new-branch-name");if(!o)return;let x=o.value.trim();if(!x){_("Please enter a branch name","warning");return}if(!confirm(`Create new branch "${x}" and switch to it?`))return;let r=new FormData;r.append("branch",x),r.append("create","true"),fetch(`/api/services/${b}/git/checkout/branch`,{method:"POST",body:r}).then((d)=>{if(!d.ok)throw new Error(`HTTP error: ${d.status}`);return d.json()}).then((d)=>{o.value="",window.htmx.ajax("GET",`/api/templates/git-section?service=${b}`,{target:"#git-section",swap:"outerHTML"}),_(`Successfully created and switched to branch "${x}"`,"success")}).catch((d)=>{console.error("Error creating branch:",d),_("Failed to create branch","error")})}function l(b){window.htmx.ajax("GET",`/api/templates/git-section?service=${b}`,{target:"#git-section",swap:"outerHTML"})}function I(b){let o=document.getElementById(b);if(!o)return;o.classList.toggle("collapsed");let x=o.classList.contains("collapsed");localStorage.setItem(P.GIT_CONTENT_COLLAPSED(b),x.toString())}function Z(){let b=localStorage.getItem(P.GIT_CONTENT_COLLAPSED("git-commits-list"))==="true",o=document.getElementById("git-commits-list");if(o&&b)o.classList.add("collapsed");let x=localStorage.getItem(P.GIT_CONTENT_COLLAPSED("git-branches-list"))==="true",r=document.getElementById("git-branches-list");if(r&&x)r.classList.add("collapsed")}function C(b,o,x,r=""){let d=P.SERVICE_CONFIG(b),u=Y(b);u[o]={envVars:x,description:r,lastUsed:new Date().toISOString(),created:u[o]?.created||new Date().toISOString()},localStorage.setItem(d,JSON.stringify(u)),console.log(`Saved configuration '${o}' for service '${b}'`)}function Y(b){let o=P.SERVICE_CONFIG(b),x=localStorage.getItem(o);return x?JSON.parse(x):{}}function T(b,o){return Y(b)[o]||null}function S(b,o){let x=P.SERVICE_CONFIG(b),r=Y(b);if(r[o])return delete r[o],localStorage.setItem(x,JSON.stringify(r)),console.log(`Deleted configuration '${o}' for service '${b}'`),!0;return!1}function g(b){let o={},x=document.getElementById(`env-vars-${b}`);if(x)x.querySelectorAll(".env-var-pair").forEach((r)=>{let d=r.querySelectorAll("input"),[u,f]=d;if(u?.value)o[u.value]=f?.value||""});return o}function U(b,o){let x=document.getElementById(`env-vars-${b}`);if(!x)return;x.innerHTML="",Object.entries(o).forEach(([r,d])=>{window.htmx.ajax("GET",`/api/templates/env-var-pair?key=${encodeURIComponent(r)}&value=${encodeURIComponent(d)}`,{target:`#env-vars-${b}`,swap:"beforeend"})}),window.htmx.ajax("GET","/api/templates/env-var-pair",{target:`#env-vars-${b}`,swap:"beforeend"})}function p(b){let o=document.getElementById(`env-vars-${b}`);if(!o)return;if(confirm("Clear all current environment variables?"))o.innerHTML="",window.htmx.ajax("GET","/api/templates/env-var-pair",{target:`#env-vars-${b}`,swap:"beforeend"}),_("Environment variables cleared","info")}function n(b){let o=document.getElementById(`management-${b}`);if(o){let x=o.style.display==="none";o.style.display=x?"block":"none"}}function t(b,o){if(!o)return;let x=T(b,o);if(x)U(b,x.envVars),console.log(`Loaded configuration '${o}' for service '${b}'`)}function a(b){let o=window.nexsock.getCurrentEnvVars(b);if(Object.keys(o).length===0){_("Please add some environment variables before saving a configuration.","warning");return}let x=prompt("Enter a name for this configuration:");if(!x)return;let r=prompt("Enter a description (optional):")||"";window.nexsock.saveServiceConfig(b,x,o,r),window.nexsock.refreshConfigUI(b),_(`Configuration '${x}' saved successfully!`,"success")}function W(b){window.htmx.ajax("GET",`/api/templates/config-section?service=${encodeURIComponent(b)}`,{target:`#config-section-${b}`,swap:"innerHTML"})}function v(b,o){if(confirm(`Are you sure you want to delete the configuration '${o}'?`))window.nexsock.deleteServiceConfig(b,o),window.htmx.ajax("GET",`/api/templates/config-modal-content?service=${encodeURIComponent(b)}`,{target:".modal-body",swap:"innerHTML"}),window.nexsock.refreshConfigUI(b),_(`Configuration '${o}' deleted successfully.`,"success")}async function N(b){if(!b){_("Invalid service name","error");return}if(confirm(`Are you sure you want to remove ${b}? This action cannot be undone.`))try{let o=await fetch(`/api/services/${b}`,{method:"DELETE"});if(!o.ok)throw new Error(`HTTP error: ${o.status}`);_(`Service '${b}' removed successfully.`,"success"),window.location.href="/"}catch(o){console.error("Error removing service:",o),_("Failed to remove service","error")}}function c(){document.querySelectorAll("[data-service-name]").forEach((b)=>{let o=b.getAttribute("data-service-name");if(o)W(o)}),Z(),document.body.addEventListener("htmx:responseError",(b)=>{console.error("HTMX Error:",b.detail),_("An error occurred while loading content","error")}),document.body.addEventListener("htmx:beforeRequest",(b)=>{let o=b.target;if(o.classList.contains("button"))o.classList.add("button-loading")}),document.body.addEventListener("htmx:afterRequest",(b)=>{let o=b.target;if(o.classList.contains("button"))o.classList.remove("button-loading");Z()}),document.addEventListener("click",(b)=>{let o=b.target;if(o.classList.contains("modal-overlay"))k();if(!o.closest(".dropdown"))Q()}),document.addEventListener("keydown",(b)=>{if(b.key==="Escape")k(),Q()})}var i=()=>{return{saveServiceConfig:C,getServiceConfigs:Y,loadServiceConfig:T,deleteServiceConfig:S,getCurrentEnvVars:g,applyEnvVarsToForm:U,loadConfigFromSelection:t,showSaveConfigModal:a,refreshConfigUI:W,deleteConfigAndRefresh:v,toggleManagement:n,closeModal:k,showMessage:_,confirmRemove:N,showGitTab:h,createNewBranch:y,refreshGitSection:l,toggleDropdown:E,closeAllDropdowns:Q,clearCurrentEnvVars:p,toggleGitContent:I,restoreGitContentVisibility:Z}};document.addEventListener("DOMContentLoaded",()=>{c(),window.nexsock=i(),M(),console.log("Nexsock web interface initialized successfully")});})();

//# debugId=7E605979DEAA11B264756E2164756E21
//# sourceMappingURL=main.js.map
