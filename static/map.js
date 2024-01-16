let map;
let markers_map = new Map();

let pin_id = 0;
let pin_map = new Map(); // id (int) to pin struct

var PinStruct = {
	marker: null,
	infowindow: null,
	initialize: function(marker, infowindow) {
	  this.marker = marker;
	  this.infowindow = infowindow;
	  return this;
	}
   };

async function loadPins() {
	return fetch('/api/pins', {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json',
		},
	})
	.then(response => {
		if(response.status !== 200) {
			throw new Error(response.status);
		}
		return response.json();
	})
	.then((data) => {
		//console.log(data);
		return data;
	})
	.catch((error) => {
		console.error('Error:', error);
		return [];
	});
}

   

function initMap() {
	map = new google.maps.Map(document.getElementById("map"), {
	center: new google.maps.LatLng(-33.91722, 151.23064),
	zoom: 16,
	disableDoubleClickZoom: true,
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
	
	google.maps.event.addListener(map, 'dblclick', function(event){
		placeMarker(event.latLng);
	});

	class NoteWindow extends google.maps.InfoWindow
	{
		myholytextelem;
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
			const divElem = document.createElement("div")
			const textNode = document.createElement("p"); 
			textNode.setAttribute("contenteditable", "true");
			textNode.textContent = "Edit me!";
			divElem.appendChild(textNode);
			const newButton = document.createElement('button');
			newButton.textContent = 'Save note!';
			newButton.addEventListener('click', () => {
				var noteData=textNode.textContent;
				console.log("this.id = ", this.id);
				fetch('/api/pin', {
					method: 'POST',
					headers: {
						'Content-Type': 'application/json',
					},
					body: JSON.stringify({	data: noteData, position: pin_map.get(this.id).marker.position}),
				})
				.then(response => response.text())
				.then((data) => {
					this.id = data.id;
					console.log(data);
				})
				.catch((error) => {
					console.error('Error:', error);
				});
				});
			divElem.appendChild(newButton);
			this.setContent(divElem);
			this.myholytextelem = textNode;
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
		infowindow.setId(pin_id);
		infowindow.init();

		var pin = Object.create(PinStruct).initialize(marker, infowindow);
		pin_map.set(pin_id, pin);

		pin_id += 1;

		marker.addListener("click", () => {
			infowindow.open({
			  anchor: marker,
			  map,
			});
			console.log(infowindow.getId());
		});

		//double click to delete pin
		marker.addListener("dblclick", function() {
			marker.setMap(null);
			markers_map.delete(marker.position);
		});

		return pin;

	}
	

	// Load user pins
	loadPins().then((data) => {
		console.log(data);
		data.forEach(element => {
			let newPin = placeMarker(element.position);
			newPin.infowindow.myholytextelem.textContent = element.data;
		});
	});
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