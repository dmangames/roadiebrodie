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

	function placeMarker(location) {
		const marker = new google.maps.Marker({
		position: location,
		icon: icons["yellow_pin"].icon,
		map: map,
		});
	}
}

window.initMap = initMap;