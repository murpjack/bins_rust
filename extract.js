// A hack to scrape some data from statically-generated web page
// Making lots of assumptions. Just prototyping
function extractJson() {
  return JSON.stringify(binDays2023());
}

function binDays2023() {
  const data = [];
  const months = document.getElementsByClassName("widget-content");
  for (var month of months) {
    var datesAndValues = month.getElementsByTagName("li");
    for (const dateAndVal of datesAndValues) {
      const [dateStr, rubbish] = dateAndVal.innerText.split(":");

      const [_, dt, mn] = dateStr.replace(/\s/g, " ").split(" ");

      const monthIdx = monthFromStr(mn);

      const date = new Date(2023, monthIdx, dt);
      const millis = Date.parse(date);

      data.push({
        date: millis,
        bins: binsFromText(rubbish),
      });
    }
  }
  return data;
}

function monthFromStr(monthStr) {
  var d = Date.parse(monthStr + " 1, 2023");
  if (!isNaN(d)) {
    return new Date(d).getMonth() + 1;
  }
  return -1;
}

function binsFromText(str) {
  var b = ["recycling", "rubbish", "food and garden waste"];
  return b.filter((val) => str.includes(val));
}
