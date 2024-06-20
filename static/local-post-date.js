addEventListener("DOMContentLoaded", (event) => {
	let postDateElements = document.getElementsByClassName("post_date");
	for (let element of postDateElements) {
		let datetime = new Date(Date.parse(element.dateTime));
		let datetime_local;
		if (datetime.getUTCHours() == 0 && datetime.getUTCMinutes() == 0 && datetime.getUTCSeconds() == 0) {
			datetime_local = datetime.toLocaleDateString();
		} else {
			datetime_local = datetime.toLocaleString();
		}
		element.innerText = datetime_local;
	}
});
