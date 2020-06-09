function compare(a, b) {
	return (
		a.i != b.i
  	&& a.nr == b.nr 
  	&& a.name == b.name 
  	&& a.days.every((d, i) => d == b.days[i])
	)
}

sections.map(s => s.children[1].querySelectorAll("table[class] > tbody")).map(a => Array.from(a).map(c => Array.from(c.querySelectorAll("td")).map(td => td.innerText)))
.map((cs, p) => 
cs.map(c => {
	let [_, name, nr_] = /([a-ząęśńćłóżź\s]+){1,}\s.*([12][0-9]{2})/i.exec(c[0]);
  let nr = +nr_;
  let days = c[4].split(", ");
	let spec = nr_[0] == "2" ? true : false;
	if (days.some(a => a=="Wt") && days.length == 1)
		return null;
	
  return {
  subject: name,
  nr,
  days,
	spec,
	period_id: p
  };
}))
.flat().filter(a => a !== null).map((a, i) => ({...a, i})).map(a => ({name: a.name, nr: a.nr}))