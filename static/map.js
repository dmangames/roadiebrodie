let map;
let markers_map = new Map();


function initMap() {
	map = new google.maps.Map(document.getElementById("map"), {
	center: new google.maps.LatLng(-33.91722, 151.23064),
	zoom: 16,
	});

	document
    .getElementById("show-markers")
    .addEventListener("click", showMarkers);
	document
    .getElementById("hide-markers")
    .addEventListener("click", hideMarkers);
	document
    .getElementById("delete-markers")
    .addEventListener("click", deleteMarkers)

	const iconBase = 'public/';

	const icons = {
	yellow_pin: {
		icon: iconBase + "rb_pin.png",
	},
	};
	
	google.maps.event.addListener(map, 'click', function(event){
		placeMarker(event.latLng);
	});

	class NoteWindow extends google.maps.InfoWindow
	{
		id = 0;
		setId(newId){
			this.id = newId;
		}
		getId(){
			return this.id;
		}
		content_changed(){
			console.log("it changed");
		}

		init(){
			const divElem = document.createElement("div");
			const node = document.createElement("p");
			node.setAttribute("contenteditable", "true");
			node.textContent = "Edit me!";
			divElem.appendChild(node);
			const newButton = document.createElement('button');
			newButton.textContent = 'Click me!';
			newButton.addEventListener('click', () => {
				console.log(this.getId());
			  });
			divElem.appendChild(newButton);
			this.setContent(divElem);
		}
	}

	const contentString =
			'<div id="content">' +
			'<p contenteditable="true">Edit this content to add your own quote</p>' +
			'<button type="button" onclick="this.getId()">Click Me!</button>' +
			"</div>";
	
	function placeMarker(location) {

		const marker = new google.maps.Marker({
		position: location,
		icon: icons["yellow_pin"].icon,
		map: map,
		});

		//set map with (latitude, longitude) as key - with parentheses around coordinate
		markers_map.set(location, marker);

		const infowindow = new NoteWindow();
		infowindow.init();
		infowindow.setId(1234);

		marker.addListener("click", () => {
			infowindow.open({
			  anchor: marker,
			  map,
			});
			console.log(infowindow.getId());
		});
	}

	
}

// Sets the map on all markers in the array.
function setMapOnAll(map) {
	for (let pair of markers_map.entries()){
		markers_map.get(pair[0]).setMap(map);
	}
  }
  
  // Removes the markers from the map, but keeps them in the array.
  function hideMarkers() {
	setMapOnAll(null);
  }
  
  // Shows any markers currently in the array.
  function showMarkers() {
	setMapOnAll(map);
  }
  
  // Deletes all markers in the array by removing references to them.
  function deleteMarkers() {
	hideMarkers();
	for (let pair of markers_map.entries()){
		markers_map.delete(pair[0]);
	}
  }

window.initMap = initMap;