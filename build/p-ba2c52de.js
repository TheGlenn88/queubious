let e,t,n=!1;const l="undefined"!=typeof window?window:{},s=l.document||{head:{}},o={t:0,l:"",jmp:e=>e(),raf:e=>requestAnimationFrame(e),ael:(e,t,n,l)=>e.addEventListener(t,n,l),rel:(e,t,n,l)=>e.removeEventListener(t,n,l),ce:(e,t)=>new CustomEvent(e,t)},r=e=>Promise.resolve(e),c=(()=>{try{return new CSSStyleSheet,!0}catch(e){}return!1})(),i=new WeakMap,a=e=>"sc-"+e.o,u={},f=e=>"object"==(e=typeof e)||"function"===e,$=(e,t,...n)=>{let l=null,s=!1,o=!1,r=[];const c=t=>{for(let n=0;n<t.length;n++)l=t[n],Array.isArray(l)?c(l):null!=l&&"boolean"!=typeof l&&((s="function"!=typeof e&&!f(l))&&(l+=""),s&&o?r[r.length-1].i+=l:r.push(s?d(null,l):l),o=s)};if(c(n),t){const e=t.className||t.class;e&&(t.class="object"!=typeof e?e:Object.keys(e).filter((t=>e[t])).join(" "))}const i=d(e,null);return i.u=t,r.length>0&&(i.$=r),i},d=(e,t)=>({t:0,m:e,i:t,p:null,$:null,u:null}),m={},p=(e,t,n,s,r,c)=>{if(n!==s){let i=N(e,t),a=t.toLowerCase();if("class"===t){const t=e.classList,l=h(n),o=h(s);t.remove(...l.filter((e=>e&&!o.includes(e)))),t.add(...o.filter((e=>e&&!l.includes(e))))}else if("ref"===t)s&&s(e);else if(i||"o"!==t[0]||"n"!==t[1]){const l=f(s);if((i||l&&null!==s)&&!r)try{if(e.tagName.includes("-"))e[t]=s;else{let l=null==s?"":s;"list"===t?i=!1:null!=n&&e[t]==l||(e[t]=l)}}catch(e){}null==s||!1===s?!1===s&&""!==e.getAttribute(t)||e.removeAttribute(t):(!i||4&c||r)&&!l&&e.setAttribute(t,s=!0===s?"":s)}else t="-"===t[2]?t.slice(3):N(l,a)?a.slice(2):a[2]+t.slice(3),n&&o.rel(e,t,n,!1),s&&o.ael(e,t,s,!1)}},y=/\s/,h=e=>e?e.split(y):[],w=(e,t,n,l)=>{const s=11===t.p.nodeType&&t.p.host?t.p.host:t.p,o=e&&e.u||u,r=t.u||u;for(l in o)l in r||p(s,l,o[l],void 0,n,t.t);for(l in r)p(s,l,o[l],r[l],n,t.t)},b=(t,n,l)=>{let o,r,c=n.$[l],i=0;if(null!==c.i)o=c.p=s.createTextNode(c.i);else if(o=c.p=s.createElement(c.m),w(null,c,!1),null!=e&&o["s-si"]!==e&&o.classList.add(o["s-si"]=e),c.$)for(i=0;i<c.$.length;++i)r=b(t,c,i),r&&o.appendChild(r);return o},g=(e,n,l,s,o,r)=>{let c,i=e;for(i.shadowRoot&&i.tagName===t&&(i=i.shadowRoot);o<=r;++o)s[o]&&(c=b(null,l,o),c&&(s[o].p=c,i.insertBefore(c,n)))},j=(e,t,n,l,s)=>{for(;t<=n;++t)(l=e[t])&&(s=l.p,M(l),s.remove())},v=(e,t)=>e.m===t.m,S=(e,t)=>{const n=t.p=e.p,l=e.$,s=t.$,o=t.i;null===o?(w(e,t,!1),null!==l&&null!==s?((e,t,n,l)=>{let s,o=0,r=0,c=t.length-1,i=t[0],a=t[c],u=l.length-1,f=l[0],$=l[u];for(;o<=c&&r<=u;)null==i?i=t[++o]:null==a?a=t[--c]:null==f?f=l[++r]:null==$?$=l[--u]:v(i,f)?(S(i,f),i=t[++o],f=l[++r]):v(a,$)?(S(a,$),a=t[--c],$=l[--u]):v(i,$)?(S(i,$),e.insertBefore(i.p,a.p.nextSibling),i=t[++o],$=l[--u]):v(a,f)?(S(a,f),e.insertBefore(a.p,i.p),a=t[--c],f=l[++r]):(s=b(t&&t[r],n,r),f=l[++r],s&&i.p.parentNode.insertBefore(s,i.p));o>c?g(e,null==l[u+1]?null:l[u+1].p,n,l,r,u):r>u&&j(t,o,c)})(n,l,t,s):null!==s?(null!==e.i&&(n.textContent=""),g(n,null,t,s,0,s.length-1)):null!==l&&j(l,0,l.length-1)):e.i!==o&&(n.data=o)},M=e=>{e.u&&e.u.ref&&e.u.ref(null),e.$&&e.$.map(M)},O=(e,t)=>{t&&!e.h&&t["s-p"]&&t["s-p"].push(new Promise((t=>e.h=t)))},k=(e,t)=>{if(e.t|=16,!(4&e.t))return O(e,e.g),Y((()=>C(e,t)));e.t|=512},C=(e,t)=>{const n=e.j;return A(void 0,(()=>P(e,n,t)))},P=async(n,l,o)=>{const r=n.v,c=r["s-rc"];o&&(e=>{const t=e.S,n=e.v,l=t.t,o=((e,t)=>{let n=a(t),l=B.get(n);if(e=11===e.nodeType?e:s,l)if("string"==typeof l){let t,o=i.get(e=e.head||e);o||i.set(e,o=new Set),o.has(n)||(t=s.createElement("style"),t.innerHTML=l,e.insertBefore(t,e.querySelector("link")),o&&o.add(n))}else e.adoptedStyleSheets.includes(l)||(e.adoptedStyleSheets=[...e.adoptedStyleSheets,l]);return n})(n.shadowRoot?n.shadowRoot:n.getRootNode(),t);10&l&&(n["s-sc"]=o,n.classList.add(o+"-h"))})(n);((n,l)=>{const s=n.v,o=n.S,r=n.M||d(null,null),c=(e=>e&&e.m===m)(l)?l:$(null,null,l);t=s.tagName,o.O&&(c.u=c.u||{},o.O.map((([e,t])=>c.u[t]=s[e]))),c.m=null,c.t|=4,n.M=c,c.p=r.p=s.shadowRoot||s,e=s["s-sc"],S(r,c)})(n,x(n,l)),c&&(c.map((e=>e())),r["s-rc"]=void 0);{const e=r["s-p"],t=()=>E(n);0===e.length?t():(Promise.all(e).then(t),n.t|=4,e.length=0)}},x=(e,t)=>{try{t=t.render(),e.t&=-17,e.t|=2}catch(t){V(t,e.v)}return t},E=e=>{const t=e.v,n=e.j,l=e.g;64&e.t||(e.t|=64,F(t),T(n,"componentDidLoad"),e.k(t),l||L()),e.C(t),e.h&&(e.h(),e.h=void 0),512&e.t&&X((()=>k(e,!1))),e.t&=-517},L=()=>{F(s.documentElement),X((()=>(e=>{const t=o.ce("appload",{detail:{namespace:"waiting-room"}});return e.dispatchEvent(t),t})(l)))},T=(e,t,n)=>{if(e&&e[t])try{return e[t](n)}catch(e){V(e)}},A=(e,t)=>e&&e.then?e.then(t):t(),F=e=>e.classList.add("hydrated"),H=(e,t,n)=>{if(t.P){const l=Object.entries(t.P),s=e.prototype;if(l.map((([e,[l]])=>{31&l||2&n&&32&l?Object.defineProperty(s,e,{get(){return((e,t)=>W(this).L.get(t))(0,e)},set(n){((e,t,n,l)=>{const s=W(e),o=s.L.get(t),r=s.t,c=s.j;n=((e,t)=>null==e||f(e)?e:2&t?parseFloat(e):1&t?e+"":e)(n,l.P[t][0]),8&r&&void 0!==o||n===o||(s.L.set(t,n),c&&2==(18&r)&&k(s,!1))})(this,e,n,t)},configurable:!0,enumerable:!0}):1&n&&64&l&&Object.defineProperty(s,e,{value(...t){const n=W(this);return n.T.then((()=>n.j[e](...t)))}})})),1&n){const n=new Map;s.attributeChangedCallback=function(e,t,l){o.jmp((()=>{const t=n.get(e);this[t]=(null!==l||"boolean"!=typeof this[t])&&l}))},e.observedAttributes=l.filter((([e,t])=>15&t[0])).map((([e,l])=>{const s=l[1]||e;return n.set(s,e),512&l[0]&&t.O.push([e,s]),s}))}}return e},R=(e,t={})=>{const n=[],r=t.exclude||[],i=l.customElements,u=s.head,f=u.querySelector("meta[charset]"),$=s.createElement("style"),d=[];let m,p=!0;Object.assign(o,t),o.l=new URL(t.resourcesUrl||"./",s.baseURI).href,e.map((e=>e[1].map((t=>{const l={t:t[0],o:t[1],P:t[2],A:t[3]};l.P=t[2],l.O=[];const s=l.o,u=class extends HTMLElement{constructor(e){super(e),D(e=this,l),1&l.t&&e.attachShadow({mode:"open"})}connectedCallback(){m&&(clearTimeout(m),m=null),p?d.push(this):o.jmp((()=>(e=>{if(0==(1&o.t)){const t=W(e),n=t.S,l=()=>{};if(!(1&t.t)){t.t|=1;{let n=e;for(;n=n.parentNode||n.host;)if(n["s-p"]){O(t,t.g=n);break}}n.P&&Object.entries(n.P).map((([t,[n]])=>{if(31&n&&e.hasOwnProperty(t)){const n=e[t];delete e[t],e[t]=n}})),(async(e,t,n,l,s)=>{if(0==(32&t.t)){{if(t.t|=32,(s=z(n)).then){const e=()=>{};s=await s,e()}s.isProxied||(H(s,n,2),s.isProxied=!0);const e=()=>{};t.t|=8;try{new s(t)}catch(e){V(e)}t.t&=-9,e()}if(s.style){let e=s.style;const t=a(n);if(!B.has(t)){const l=()=>{};((e,t,n)=>{let l=B.get(e);c&&n?(l=l||new CSSStyleSheet,l.replace(t)):l=t,B.set(e,l)})(t,e,!!(1&n.t)),l()}}}const o=t.g,r=()=>k(t,!0);o&&o["s-rc"]?o["s-rc"].push(r):r()})(0,t,n)}l()}})(this)))}disconnectedCallback(){o.jmp((()=>{}))}componentOnReady(){return W(this).F}};l.H=e[0],r.includes(s)||i.get(s)||(n.push(s),i.define(s,H(u,l,1)))})))),$.innerHTML=n+"{visibility:hidden}.hydrated{visibility:inherit}",$.setAttribute("data-styles",""),u.insertBefore($,f?f.nextSibling:u.firstChild),p=!1,d.length?d.map((e=>e.connectedCallback())):o.jmp((()=>m=setTimeout(L,30)))},U=new WeakMap,W=e=>U.get(e),q=(e,t)=>U.set(t.j=e,t),D=(e,t)=>{const n={t:0,v:e,S:t,L:new Map};return n.T=new Promise((e=>n.C=e)),n.F=new Promise((e=>n.k=e)),e["s-p"]=[],e["s-rc"]=[],U.set(e,n)},N=(e,t)=>t in e,V=(e,t)=>(0,console.error)(e,t),_=new Map,z=e=>{const t=e.o.replace(/-/g,"_"),n=e.H,l=_.get(n);return l?l[t]:import(`./${n}.entry.js`).then((e=>(_.set(n,e),e[t])),V)},B=new Map,G=[],I=[],J=(e,t)=>l=>{e.push(l),n||(n=!0,t&&4&o.t?X(Q):o.raf(Q))},K=e=>{for(let t=0;t<e.length;t++)try{e[t](performance.now())}catch(e){V(e)}e.length=0},Q=()=>{K(G),K(I),(n=G.length>0)&&o.raf(Q)},X=e=>r().then(e),Y=J(I,!0);export{m as H,R as b,$ as h,r as p,q as r}