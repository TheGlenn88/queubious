import{r as s,h as t,H as a}from"./p-6291a37a.js";const r=class{constructor(t){s(this,t)}fetchData(){var s;(s=this.statusUrl,fetch(s).then((s=>s.json()))).then((s=>{this.position=s.position}))}async refreshQueueData(){this.fetchData()}render(){return t(a,null,t("div",null,"Queue position ",this.position))}};r.style=":host{display:block}";export{r as waiting_room}