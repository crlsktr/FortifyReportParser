use std::fs;
fn main() {
	let filename = std::env::args().nth(1).expect("No file name given");
	let issues = fs::read_to_string(filename).expect("Something went wrong trying to read the file");
	let doc = roxmltree::Document::parse(issues.as_str()).unwrap();
	let xml_iterator = doc.descendants().filter(|n| n.tag_name().name() == "Issue");
	
	for i in xml_iterator
	{
		let iid = match i.attribute("iid") {
			Some(x) => x,
			None => ""
		};

		let category = match i.attribute("category") {
			Some(c) => c,
			None => ""
		};

		let rule_id = match i.attribute("ruleID") {
			Some(r) => r,
			None => ""
		};

		println!("Rule:{:?}<IID>{:?}</IID> cat:{:?}",rule_id, iid, category);
		// println!("{:?}", i);
	}
}
