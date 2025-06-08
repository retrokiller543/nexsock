(()=>{var kx=Object.defineProperty;var Kx=(x,b)=>{for(var z in b)kx(x,z,{get:b[z],enumerable:!0,configurable:!0,set:($)=>b[z]=()=>$})};var Xx=(x,b)=>()=>(x&&(b=x(x=0)),b);var T={};Kx(T,{ErrorModal:()=>P});function U(x){let{errorDetails:b,onClose:z}=x,$=()=>{if(z)z();else{let d=document.querySelector(".error-modal-overlay");if(d)d.remove()}},Q=()=>{if(b.fullErrorPageUrl)window.open(b.fullErrorPageUrl,"_blank")},Z=()=>{if(b.originalUrl)window.navigateToErrorPage(b.originalUrl)},_=()=>{return!!(new URLSearchParams(window.location.search).has("debug")||localStorage.getItem("nexsock-debug")==="true"||window.location.hostname==="localhost"||window.location.hostname==="127.0.0.1"||window.NEXSOCK_DEBUG===!0)};return createElement("div",{className:"error-modal-overlay modal-overlay",onClick:(d)=>{if(d.target.classList.contains("error-modal-overlay"))$()}},createElement("div",{className:"error-modal modal"},createElement("div",{className:"error-modal-header"},createElement("h2",null,"\uD83D\uDEA8 Error Details"),createElement("button",{className:"close-button",onClick:$},"×")),createElement("div",{className:"error-modal-body"},createElement("div",{className:"error-code"},b.errorCode),createElement("div",{className:"error-message"},b.errorMessage),createElement("div",{className:"error-diagnostics"},createElement("h3",null,"\uD83D\uDD0D Diagnostics"),createElement("div",{className:"diagnostics-content",innerHTML:b.diagnostics})),b.debugInfo&&createElement("div",{className:"error-debug"},createElement("button",{className:"debug-toggle",onClick:()=>{let d=document.querySelectorAll(".debug-toggle"),J=d[d.length-1],f=J.nextElementSibling;if(f.style.display==="none")f.style.display="block",J.textContent="\uD83D\uDC1B Hide Debug Info";else f.style.display="none",J.textContent="\uD83D\uDC1B Show Debug Info"}},"\uD83D\uDC1B Show Debug Info"),createElement("div",{className:"debug-content",style:{display:"none"}},createElement("pre",null,b.debugInfo)))),createElement("div",{className:"error-modal-footer"},b.fullErrorPageUrl&&createElement("button",{className:"button button-primary",onClick:Q},"\uD83D\uDD0D View Full Error Page"),_()&&b.originalUrl&&createElement("button",{className:"button button-warning",onClick:Z},"\uD83D\uDEA7 Debug Mode: Go to Error Page"),createElement("button",{className:"button button-secondary",onClick:$},"Close"))))}var Rx=`
.error-modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.6);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  backdrop-filter: blur(2px);
}

.error-modal {
  background: var(--surface-color);
  border-radius: 12px;
  border: 1px solid var(--error-color);
  max-width: 90vw;
  max-height: 90vh;
  width: 800px;
  overflow: hidden;
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.3);
  animation: errorModalSlideIn 0.3s ease-out;
}

@keyframes errorModalSlideIn {
  from {
    opacity: 0;
    transform: translateY(-20px) scale(0.95);
  }
  to {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
}

.error-modal-header {
  background: linear-gradient(90deg, var(--error-color) 0%, var(--error-hover-color) 100%);
  color: var(--surface-color);
  padding: 20px 30px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  border-bottom: 1px solid var(--border-color);
}

.error-modal-header h2 {
  margin: 0;
  font-size: 1.3em;
  font-weight: 600;
}

.close-button {
  background: none;
  border: none;
  color: var(--surface-color);
  font-size: 1.5em;
  cursor: pointer;
  padding: 0;
  width: 30px;
  height: 30px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  transition: background-color 0.2s ease;
}

.close-button:hover {
  background: rgba(255, 255, 255, 0.2);
}

.error-modal-body {
  padding: 30px;
  overflow-y: auto;
  max-height: 60vh;
}

.error-code {
  background: var(--secondary-bg-color);
  color: var(--warning-color);
  padding: 6px 12px;
  border-radius: 6px;
  font-size: 0.9em;
  font-weight: bold;
  display: inline-block;
  margin-bottom: 15px;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
}

.error-message {
  font-size: 1.1em;
  margin-bottom: 25px;
  color: var(--text-color);
  line-height: 1.5;
}

.error-diagnostics {
  margin: 20px 0;
}

.error-diagnostics h3 {
  color: var(--primary-color);
  margin-bottom: 15px;
  font-size: 1.1em;
}

.diagnostics-content {
  background: var(--secondary-bg-color);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  padding: 20px;
  overflow-x: auto;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 0.9em;
  line-height: 1.4;
  color: var(--text-color);
}

.error-debug {
  margin-top: 20px;
}

.debug-toggle {
  background: var(--secondary-color);
  color: var(--text-color);
  border: none;
  padding: 8px 16px;
  border-radius: 6px;
  cursor: pointer;
  font-family: inherit;
  font-size: 0.9em;
  transition: all 0.2s ease;
  margin-bottom: 15px;
}

.debug-toggle:hover {
  background: var(--secondary-hover-color);
}

.debug-content pre {
  background: var(--secondary-bg-color);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  padding: 15px;
  overflow-x: auto;
  font-size: 0.85em;
  color: var(--muted-text-color);
  white-space: pre-wrap;
  margin: 0;
}

.error-modal-footer {
  padding: 20px 30px;
  background: var(--secondary-bg-color);
  border-top: 1px solid var(--border-color);
  display: flex;
  justify-content: flex-end;
  gap: 10px;
}

.button {
  padding: 8px 16px;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 0.9em;
  font-weight: 500;
  transition: all 0.2s ease;
  text-decoration: none;
  display: inline-flex;
  align-items: center;
  gap: 6px;
}

.button-primary {
  background: var(--primary-color);
  color: var(--surface-color);
}

.button-primary:hover {
  background: var(--primary-hover-color);
  transform: translateY(-1px);
}

.button-warning {
  background: var(--warning-color);
  color: var(--surface-color);
}

.button-warning:hover {
  background: var(--warning-hover-color, #f59e0b);
  transform: translateY(-1px);
}

.button-secondary {
  background: var(--secondary-color);
  color: var(--text-color);
  border: 1px solid var(--border-color);
}

.button-secondary:hover {
  background: var(--secondary-hover-color);
}
`,P;var H=Xx(()=>{U.css=Rx;P=U});var W=null;function A(x){return`nx-${x}-${Math.random().toString(36).substring(2,8)}`}function G(x,b){if(!x||!b)return;let z=x.replace(/([^{}]+){/g,(Q,Z)=>{let _=Z.trim();if(_.startsWith("@")||_.includes(`[data-scope="${b}"]`))return Q;return`${_.split(",").map((J)=>{let f=J.trim();return`[data-scope="${b}"] ${f}`}).join(", ")} {`}),$=document.createElement("style");$.textContent=z,$.setAttribute("data-scope-id",b),document.head.appendChild($)}function Lx(x,b,...z){if(typeof x==="function"){if(x&&typeof x==="object"&&"component"in x&&"css"in x){let Q=x,Z=Q.component.name||"Component",_=A(Z);G(Q.css,_);let d=W;W=_;let J=Q.component({...b,children:z});return W=d,J}if(x.css){let Q=x.name||"Component",Z=A(Q);G(x.css,Z);let _=W;W=Z;let d=x({...b,children:z});return W=_,d}return x({...b,children:z})}let $=document.createElement(x);if(W)$.setAttribute("data-scope",W);if(b)Object.entries(b).forEach(([Q,Z])=>{if(Q==="className")$.className=Z;else if(Q==="css"){let _=A("inline");G(Z,_),$.setAttribute("data-scope",_)}else if(Q==="innerHTML")$.innerHTML=Z;else if(Q==="style"&&typeof Z==="object")Object.assign($.style,Z);else if(Q.startsWith("on")&&typeof Z==="function"){let _=Q.toLowerCase().slice(2);$.addEventListener(_,Z)}else if(typeof Z==="string"||typeof Z==="number")$.setAttribute(Q,Z.toString())});return z.flat().forEach((Q)=>{if(typeof Q==="string"||typeof Q==="number")$.appendChild(document.createTextNode(String(Q)));else if(Q instanceof Node)$.appendChild(Q)}),$}function Hx({children:x}){let b=document.createDocumentFragment();return x.flat().forEach((z)=>{if(typeof z==="string"||typeof z==="number")b.appendChild(document.createTextNode(String(z)));else if(z instanceof Node)b.appendChild(z)}),b}globalThis.createElement=Lx;globalThis.Fragment=Hx;H();var u=`.ns-card {
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
}`;function I(x){let{children:b,title:z,subtitle:$,footer:Q,variant:Z="default",className:_=""}=x,d=["ns-card",Z!=="default"&&Z,_].filter(Boolean).join(" ");return createElement("div",{className:d},(z||$)&&createElement("div",{className:"ns-card-header"},z&&createElement("h3",{className:"ns-card-title"},z),$&&createElement("p",{className:"ns-card-subtitle"},$)),b&&createElement("div",{className:"ns-card-body"},b),Q&&createElement("div",{className:"ns-card-footer"},Q))}I.css=u;var M=I;var S=`.ns-badge {
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
}`;function D(x){let{children:b,variant:z="neutral",size:$="medium",style:Q="default",className:Z=""}=x,_=["ns-badge",z,$!=="medium"&&$,Q!=="default"&&Q,Z].filter(Boolean).join(" ");return createElement("span",{className:_},b)}D.css=S;var E=D;function p(x){let{errorDetails:b,onViewDetails:z,onClose:$}=x,Q=()=>{if(z)z();else Promise.resolve().then(() => (H(),T)).then(({ErrorModal:_})=>{let d=_({errorDetails:b});document.body.appendChild(d)})},Z=()=>{if($)$();else{let _=document.querySelector(".enhanced-error");if(_&&_.parentNode)_.parentNode.removeChild(_)}};return createElement("div",{className:"message message-error enhanced-error"},createElement("div",{className:"error-summary"},createElement("strong",null,b.errorCode),createElement("p",null,b.errorMessage),createElement("div",{className:"error-actions"},createElement("button",{className:"error-details-button",onClick:Q},"View Details"),createElement("button",{className:"error-close-button",onClick:Z},"×"))))}var Bx=`
.enhanced-error {
  border-left: 4px solid var(--error-color);
  background: var(--surface-color);
  border: 1px solid var(--error-color);
  border-radius: 8px;
  padding: 16px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  animation: messageSlideIn 0.3s ease-out;
  position: relative;
}

@keyframes messageSlideIn {
  from {
    opacity: 0;
    transform: translateX(100%);
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}

.error-summary {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.error-summary strong {
  color: var(--error-color);
  font-size: 0.9em;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-weight: bold;
}

.error-summary p {
  margin: 0;
  color: var(--text-color);
  line-height: 1.4;
  font-size: 0.95em;
}

.error-actions {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: 8px;
}

.error-details-button {
  background: var(--error-color);
  color: var(--surface-color);
  border: none;
  padding: 6px 12px;
  border-radius: 4px;
  font-size: 0.85em;
  cursor: pointer;
  transition: all 0.2s ease;
  font-weight: 500;
}

.error-details-button:hover {
  background: var(--error-hover-color);
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

.error-close-button {
  background: none;
  border: none;
  color: var(--muted-text-color);
  font-size: 1.2em;
  cursor: pointer;
  padding: 4px;
  border-radius: 50%;
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
}

.error-close-button:hover {
  background: var(--secondary-bg-color);
  color: var(--text-color);
}

/* Messages container positioning */
.messages {
  position: fixed;
  top: 80px;
  right: 20px;
  max-width: 400px;
  z-index: 999;
  display: flex;
  flex-direction: column;
  gap: 10px;
}

/* Responsive design */
@media (max-width: 768px) {
  .messages {
    right: 10px;
    left: 10px;
    max-width: none;
  }
  
  .enhanced-error {
    font-size: 0.9em;
  }
  
  .error-actions {
    flex-direction: column;
    align-items: stretch;
    gap: 8px;
  }
  
  .error-close-button {
    position: absolute;
    top: 8px;
    right: 8px;
  }
}
`;p.css=Bx;var R=p;var C=`.ns-button {
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
}`;function h(x){let{children:b,variant:z="primary",size:$="medium",disabled:Q=!1,loading:Z=!1,type:_="button",onClick:d,href:J,target:f,className:jx=""}=x,y=["ns-button",z,$,Z&&"loading",jx].filter(Boolean).join(" ");if(J)return createElement("a",{href:J,target:f,className:y,onClick:d,"aria-disabled":Q},b);return createElement("button",{type:_,className:y,...Q?{disabled:!0}:{},onClick:d},b)}h.css=C;var v=h;function Fx(x,b){if(b.css&&!document.querySelector(`style[data-component="${x}"]`)){let $=document.createElement("style");$.textContent=b.css,$.setAttribute("data-component",x),document.head.appendChild($)}class z extends HTMLElement{mounted=!1;static get observedAttributes(){return b.observedAttributes||[]}connectedCallback(){this.render(),this.mounted=!0}attributeChangedCallback(){if(this.mounted)this.render()}render(){let $={};for(let d=0;d<this.attributes.length;d++){let J=this.attributes[d];if(!J)continue;let f=J.value;if(f.startsWith("{")||f.startsWith("[")||f==="true"||f==="false")try{f=JSON.parse(f)}catch{}$[this.camelCase(J.name)]=f}for(let d in $)if(d.startsWith("on")&&typeof $[d]==="string"){let J=d.slice(2).toLowerCase(),f=$[d];if(f.includes("alert")||f.includes("console"))$[d]=new Function("event",f)}let Q=this.innerHTML,Z=[];if(Q.trim()){let d=document.createElement("div");d.innerHTML=Q,Z.push(...Array.from(d.childNodes))}if(Z.length>0)$.children=Z;this.innerHTML="";let _=b.component($);if(_ instanceof Node)this.appendChild(_)}camelCase($){return $.replace(/-([a-z])/g,(Q,Z)=>Z.toUpperCase())}}customElements.define(x,z)}function N(x,b){Fx(x,{component:b,css:b.css,observedAttributes:["*"]})}var g={"error-modal":P,"ns-card":M,"ns-badge":E,"error-notification":R,"ns-button":v};function o(){Object.entries(g).forEach(([x,b])=>{N(x,b)}),console.log("Auto-registered components:",Object.keys(g))}function Y(x,b="info"){let z=document.createElement("div");z.className=`message message-${b}`,z.textContent=x;let $=document.getElementById("messages-container");if(!$)$=document.createElement("div"),$.id="messages-container",$.className="messages",document.body.appendChild($);$.appendChild(z),setTimeout(()=>{if(z.parentNode)z.parentNode.removeChild(z)},5000)}function j(){let x=document.querySelector(".modal-overlay");if(x)x.remove()}function l(x){let b=document.getElementById(x);if(!b)return;document.querySelectorAll(".dropdown.active").forEach((z)=>{if(z.id!==x)z.classList.remove("active")}),b.classList.toggle("active")}function k(){document.querySelectorAll(".dropdown.active").forEach((x)=>{x.classList.remove("active")})}var O={SERVICE_CONFIG:(x)=>`nexsock_service_config_${x}`,GIT_CONTENT_COLLAPSED:(x)=>`git_${x}_collapsed`};function r(x,b){document.querySelectorAll(".tab-button").forEach((Q)=>{Q.classList.remove("active")});let z=event?.target;if(z)z.classList.add("active");let $=document.getElementById("git-tab-content");if(!$)return;if(x==="commits")$.innerHTML='<div class="loading">Loading commits...</div>',window.htmx.ajax("GET",`/api/templates/git-log?service=${b}`,{target:"#git-tab-content",swap:"innerHTML"});else if(x==="branches")$.innerHTML='<div class="loading">Loading branches...</div>',window.htmx.ajax("GET",`/api/templates/git-branches?service=${b}`,{target:"#git-tab-content",swap:"innerHTML"})}function a(x){let b=document.getElementById("new-branch-name");if(!b)return;let z=b.value.trim();if(!z){Y("Please enter a branch name","warning");return}if(!confirm(`Create new branch "${z}" and switch to it?`))return;let $=new FormData;$.append("branch",z),$.append("create","true"),fetch(`/api/services/${x}/git/checkout/branch`,{method:"POST",body:$}).then((Q)=>{if(!Q.ok)throw new Error(`HTTP error: ${Q.status}`);return Q.json()}).then((Q)=>{b.value="",window.htmx.ajax("GET",`/api/templates/git-section?service=${x}`,{target:"#git-section",swap:"outerHTML"}),Y(`Successfully created and switched to branch "${z}"`,"success")}).catch((Q)=>{console.error("Error creating branch:",Q),Y("Failed to create branch","error")})}function c(x){window.htmx.ajax("GET",`/api/templates/git-section?service=${x}`,{target:"#git-section",swap:"outerHTML"})}function n(x){let b=document.getElementById(x);if(!b)return;b.classList.toggle("collapsed");let z=b.classList.contains("collapsed");localStorage.setItem(O.GIT_CONTENT_COLLAPSED(x),z.toString())}function K(){let x=localStorage.getItem(O.GIT_CONTENT_COLLAPSED("git-commits-list"))==="true",b=document.getElementById("git-commits-list");if(b&&x)b.classList.add("collapsed");let z=localStorage.getItem(O.GIT_CONTENT_COLLAPSED("git-branches-list"))==="true",$=document.getElementById("git-branches-list");if($&&z)$.classList.add("collapsed")}var qx={light:"☀️",dark:"\uD83C\uDF19",auto:"\uD83D\uDCBB"};class m{currentTheme="auto";isInitialized=!1;constructor(){if(!this.isInitialized)this.initializeTheme(),this.setupThemeToggle(),this.isInitialized=!0}setTheme(x){if(!this.isValidTheme(x)){console.warn(`Invalid theme: ${x}`);return}this.currentTheme=x,localStorage.setItem("nexsock-theme",x),this.applyTheme(),this.updateThemeIcon()}getCurrentTheme(){return this.currentTheme}getEffectiveTheme(){if(this.currentTheme==="auto")return window.matchMedia("(prefers-color-scheme: dark)").matches?"dark":"light";return this.currentTheme}initializeTheme(){let x=localStorage.getItem("nexsock-theme");if(x&&this.isValidTheme(x))this.currentTheme=x;else this.currentTheme="auto";this.applyTheme(),this.updateThemeIcon()}setupThemeToggle(){let x=document.getElementById("theme-toggle");if(x&&!x.hasAttribute("data-theme-listener")){let b=()=>{this.toggleTheme()};x.addEventListener("click",b),x.setAttribute("data-theme-listener","true")}}toggleTheme(){let x=["light","dark","auto"],z=(x.indexOf(this.currentTheme)+1)%x.length,$=x[z];if($)this.setTheme($)}applyTheme(){document.documentElement.setAttribute("data-theme",this.currentTheme)}updateThemeIcon(){let x=document.querySelector(".theme-icon");if(x)x.textContent=qx[this.currentTheme];let b=document.getElementById("theme-toggle");if(b)b.setAttribute("aria-label",`Current theme: ${this.currentTheme}. Click to change theme.`)}isValidTheme(x){return["light","dark","auto"].includes(x)}}var B=null;function t(){if(!B)B=new m;return B}function V(){return B}H();function Ax(x){let b=document.querySelector(".error-modal-overlay");if(b)b.remove();let z=P({errorDetails:x,onClose:()=>{let $=document.querySelector(".error-modal-overlay");if($)$.remove()}});document.body.appendChild(z)}function Gx(x){let b=wx(),z=R({errorDetails:x,onViewDetails:()=>Ax(x),onClose:()=>{let $=z;if($.parentNode)$.parentNode.removeChild($)}});b.appendChild(z),setTimeout(()=>{if(z.parentNode)z.parentNode.removeChild(z)},1e4)}function Vx(x){try{let z=new DOMParser().parseFromString(x,"text/html"),$=z.querySelector(".error-code"),Q=z.querySelector(".error-message"),Z=z.querySelector(".error-details"),_=z.querySelector(".debug-output");if(!$||!Q)return null;return{errorCode:$.textContent?.trim()||"UNKNOWN_ERROR",errorMessage:Q.textContent?.trim()||"An unknown error occurred",diagnostics:Z?.innerHTML||"No diagnostic information available",debugInfo:_?.textContent||void 0}}catch(b){return console.error("Failed to parse error response:",b),null}}function wx(){let x=document.getElementById("messages-container");if(!x){x=document.createElement("div"),x.id="messages-container",x.className="messages";let b=document.querySelector("main");if(b&&b.parentNode)b.parentNode.insertBefore(x,b);else document.body.appendChild(x)}return x}function yx(){return new URLSearchParams(window.location.search).has("debug")||localStorage.getItem("nexsock-debug")==="true"||window.location.hostname==="localhost"||window.location.hostname==="127.0.0.1"||window.NEXSOCK_DEBUG===!0}function s(x){let b=new URL(x,window.location.origin);return b.searchParams.set("debug-error","true"),b.toString()}function i(x){if(x)window.location.href=s(x);else Y("No original URL available for error page navigation","warning")}function e(x,b){if(yx()){if(localStorage.getItem("nexsock-debug-auto-redirect")==="true"&&b){console.log("Debug mode: Auto-redirecting to error page"),i(b);return}}Ux(x,b)}function Ux(x,b){if(x.responseText){let Q=Vx(x.responseText);if(Q){Q.originalUrl=b,Q.fullErrorPageUrl=b?s(b):void 0,Gx(Q);return}}let z="An error occurred while processing your request",$="error";switch(x.status){case 400:z="Bad request - please check your input and try again";break;case 401:z="Authentication required - please log in";break;case 403:z="Access denied - you don't have permission for this action";break;case 404:z="The requested resource was not found";break;case 429:z="Too many requests - please wait a moment and try again",$="warning";break;case 500:z="Internal server error - please try again later";break;case 502:case 503:case 504:z="Service temporarily unavailable - please try again later";break;default:if(x.status>=400)z=`Request failed with status ${x.status}`}Y(z,$)}window.navigateToErrorPage=i;function xx(){t(),K(),document.body.addEventListener("htmx:responseError",(x)=>{let b=x;console.error("HTMX Error:",b.detail);let z=b.detail.xhr,$=b.detail.requestConfig?.path;if(z)e(z,$);else Y("An error occurred while loading content","error")}),document.body.addEventListener("htmx:pushedIntoHistory",(x)=>{let b=V();if(b){let z=b.getCurrentTheme();document.documentElement.setAttribute("data-theme",z)}}),document.body.addEventListener("htmx:beforeRequest",(x)=>{let b=x.target;if(b.classList.contains("button"))b.classList.add("button-loading")}),document.body.addEventListener("htmx:afterRequest",(x)=>{let b=x.target;if(b.classList.contains("button"))b.classList.remove("button-loading");K()}),document.body.addEventListener("htmx:afterSettle",(x)=>{let b=V();if(b){let z=b.getCurrentTheme();document.documentElement.setAttribute("data-theme",z)}}),document.addEventListener("click",(x)=>{let b=x.target;if(b.classList.contains("modal-overlay"))j();if(!b.closest(".dropdown"))k()}),document.addEventListener("keydown",(x)=>{if(x.key==="Escape")j(),k()})}function bx(x,b,z,$=""){let Q=O.SERVICE_CONFIG(x),Z=X(x);Z[b]={envVars:z,description:$,lastUsed:new Date().toISOString(),created:Z[b]?.created||new Date().toISOString()},localStorage.setItem(Q,JSON.stringify(Z)),console.log(`Saved configuration '${b}' for service '${x}'`)}function X(x){let b=O.SERVICE_CONFIG(x),z=localStorage.getItem(b);return z?JSON.parse(z):{}}function F(x,b){return X(x)[b]||null}function zx(x,b){let z=O.SERVICE_CONFIG(x),$=X(x);if($[b])return delete $[b],localStorage.setItem(z,JSON.stringify($)),console.log(`Deleted configuration '${b}' for service '${x}'`),!0;return!1}function $x(x){let b={},z=document.getElementById(`env-vars-${x}`);if(z)z.querySelectorAll(".env-var-pair").forEach(($)=>{let Q=$.querySelectorAll("input"),[Z,_]=Q;if(Z?.value)b[Z.value]=_?.value||""});return b}function q(x,b){let z=document.getElementById(`env-vars-${x}`);if(!z)return;z.innerHTML="",Object.entries(b).forEach(([$,Q])=>{window.htmx.ajax("GET",`/api/templates/env-var-pair?key=${encodeURIComponent($)}&value=${encodeURIComponent(Q)}`,{target:`#env-vars-${x}`,swap:"beforeend"})}),window.htmx.ajax("GET","/api/templates/env-var-pair",{target:`#env-vars-${x}`,swap:"beforeend"})}function Qx(x){let b=document.getElementById(`env-vars-${x}`);if(!b)return;if(confirm("Clear all current environment variables?"))b.innerHTML="",window.htmx.ajax("GET","/api/templates/env-var-pair",{target:`#env-vars-${x}`,swap:"beforeend"}),Y("Environment variables cleared","info")}function Zx(x){let b=document.getElementById(`management-${x}`);if(b){let z=b.style.display==="none";b.style.display=z?"block":"none"}}function _x(x,b){if(!b)return;let z=F(x,b);if(z)q(x,z.envVars),console.log(`Loaded configuration '${b}' for service '${x}'`)}function dx(x){let b=window.nexsock.getCurrentEnvVars(x);if(Object.keys(b).length===0){Y("Please add some environment variables before saving a configuration.","warning");return}let z=prompt("Enter a name for this configuration:");if(!z)return;let $=prompt("Enter a description (optional):")||"";window.nexsock.saveServiceConfig(x,z,b,$),window.nexsock.refreshConfigUI(x),Y(`Configuration '${z}' saved successfully!`,"success")}function Yx(x){window.htmx.ajax("GET",`/api/templates/config-section?service=${encodeURIComponent(x)}`,{target:`#config-section-${x}`,swap:"innerHTML"})}function fx(x,b){if(confirm(`Are you sure you want to delete the configuration '${b}'?`))window.nexsock.deleteServiceConfig(x,b),window.htmx.ajax("GET",`/api/templates/config-modal-content?service=${encodeURIComponent(x)}`,{target:".modal-body",swap:"innerHTML"}),window.nexsock.refreshConfigUI(x),Y(`Configuration '${b}' deleted successfully.`,"success")}async function Jx(x){if(!x){Y("Invalid service name","error");return}if(confirm(`Are you sure you want to remove ${x}? This action cannot be undone.`))try{let b=await fetch(`/api/services/${x}`,{method:"DELETE"});if(!b.ok)throw new Error(`HTTP error: ${b.status}`);Y(`Service '${x}' removed successfully.`,"success"),window.location.href="/"}catch(b){console.error("Error removing service:",b),Y("Failed to remove service","error")}}var Wx={enabled:!1,autoRedirectToErrorPage:!1,verboseLogging:!1};function L(){try{let x=localStorage.getItem("nexsock-debug-config");if(x)return{...Wx,...JSON.parse(x)}}catch(x){console.warn("Failed to parse debug config from localStorage:",x)}return Wx}function w(x){let z={...L(),...x};try{localStorage.setItem("nexsock-debug-config",JSON.stringify(z)),localStorage.setItem("nexsock-debug",z.enabled.toString()),localStorage.setItem("nexsock-debug-auto-redirect",z.autoRedirectToErrorPage.toString()),console.log("Debug config updated:",z)}catch($){console.error("Failed to save debug config:",$)}}function Tx(x={}){w({enabled:!0,...x}),console.log("\uD83D\uDEA7 Debug mode enabled for Nexsock"),console.log("To disable: nexsock.debug.disable()"),console.log("To configure: nexsock.debug.configure({ autoRedirectToErrorPage: true })")}function ux(){w({enabled:!1,autoRedirectToErrorPage:!1,verboseLogging:!1}),console.log("Debug mode disabled for Nexsock")}function Ix(x){let b=L();if(!b.enabled){console.warn("Debug mode is not enabled. Enable it first with nexsock.debug.enable()");return}w(x),console.log("Debug configuration updated:",{...b,...x})}function Mx(x,...b){let z=L();if(z.enabled&&z.verboseLogging)console.log(`[Nexsock Debug] ${x}`,...b)}function Sx(){let x=L();if(console.group("\uD83D\uDEA7 Nexsock Debug Status"),console.log("Enabled:",x.enabled),console.log("Auto-redirect to error page:",x.autoRedirectToErrorPage),console.log("Verbose logging:",x.verboseLogging),console.groupEnd(),x.enabled)console.log(`
Available debug commands:`),console.log("- nexsock.debug.disable() - Disable debug mode"),console.log("- nexsock.debug.configure({ autoRedirectToErrorPage: true }) - Auto-redirect to error pages"),console.log("- nexsock.debug.configure({ verboseLogging: true }) - Enable verbose logging"),console.log("- nexsock.debug.testError() - Trigger a test error to see error handling");else console.log("Enable debug mode with: nexsock.debug.enable()")}function Dx(){console.log("Triggering test error..."),fetch("/api/test-query-error").then((x)=>{if(!x.ok)console.log("Test error response received:",x.status)}).catch((x)=>{console.error("Test error triggered:",x)})}var Ox={enable:Tx,disable:ux,configure:Ix,status:Sx,testError:Dx,getConfig:L,log:Mx};var Px=()=>{return{saveServiceConfig:bx,getServiceConfigs:X,loadServiceConfig:F,deleteServiceConfig:zx,getCurrentEnvVars:$x,applyEnvVarsToForm:q,loadConfigFromSelection:_x,showSaveConfigModal:dx,refreshConfigUI:Yx,deleteConfigAndRefresh:fx,toggleManagement:Zx,closeModal:j,showMessage:Y,confirmRemove:Jx,showGitTab:r,createNewBranch:a,refreshGitSection:c,toggleDropdown:l,closeAllDropdowns:k,clearCurrentEnvVars:Qx,toggleGitContent:n,restoreGitContentVisibility:K,debug:Ox}};document.addEventListener("DOMContentLoaded",()=>{xx(),window.nexsock=Px(),o(),console.log("Nexsock web interface initialized successfully")});})();

//# debugId=A3BD6DE06F04BE5064756E2164756E21
//# sourceMappingURL=main.js.map
