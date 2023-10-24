let map;

function initMap() {
	map = new google.maps.Map(document.getElementById("map"), {
	center: new google.maps.LatLng(-33.91722, 151.23064),
	zoom: 16,
	});

	const iconBase = 'public/';

	const icons = {
	yellow_pin: {
		icon: iconBase + "rb_pin.png",
	},
	};
	
	google.maps.event.addListener(map, 'click', function(event){
		placeMarker(event.latLng);
	});

	const contentString =
		'<div id="content">' +
		'<div id="siteNotice">' +
		"</div>" +
		'<h1 id="firstHeading" class="firstHeading">Uluru</h1>' +
		'<div id="bodyContent">' +
		"<p><b>Uluru</b>, also referred to as <b>Ayers Rock</b>, is a large " +
		"sandstone rock formation in the southern part of the " +
		"Northern Territory, central Australia. It lies 335&#160;km (208&#160;mi) " +
		"south west of the nearest large town, Alice Springs; 450&#160;km " +
		"(280&#160;mi) by road. Kata Tjuta and Uluru are the two major " +
		"features of the Uluru - Kata Tjuta National Park. Uluru is " +
		"sacred to the Pitjantjatjara and Yankunytjatjara, the " +
		"Aboriginal people of the area. It has many springs, waterholes, " +
		"rock caves and ancient paintings. Uluru is listed as a World " +
		"Heritage Site.</p>" +
		'<p>Attribution: Uluru, <a href="https://en.wikipedia.org/w/index.php?title=Uluru&oldid=297882194">' +
		"https://en.wikipedia.org/w/index.php?title=Uluru</a> " +
		"(last visited June 22, 2009).</p>" +
		"</div>" +
		"</div>";
	
	function placeMarker(location) {
		const marker = new google.maps.Marker({
		position: location,
		icon: icons["yellow_pin"].icon,
		map: map,
		});

		const infowindow = new google.maps.InfoWindow({
			content: contentString,
			ariaLabel: "Uluru",
		});

		marker.addListener("click", () => {
			infowindow.open({
			  anchor: marker,
			  map,
			});
		});
	}
}

window.initMap = initMap;