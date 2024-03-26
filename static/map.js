let map;

let pin_id = 0;
let fake_id = "temp" + pin_id;
let pin_map = new Map(); // id (int) to pin struct <db_id, pin_struct>

var ROUTE_DISTANCE_DIVISOR = 13000; // route distance in m / this == distance from route to search
var boxes = null;
var routeBoxer = null;

document.addEventListener('DOMContentLoaded', burgerMenuActions);

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
	center: new google.maps.LatLng(37.7749, -122.4194),
	zoom: 8,
	disableDoubleClickZoom: true,
	});

	routeBoxer = new RouteBoxer();
	console.log("RouteBoxer init: ", routeBoxer);

	var show_marker_elem = document.getElementById("show-markers");
	if(show_marker_elem)
	{
		show_marker_elem.addEventListener("click", showMarkers);
	}
	var hide_markers_elem = document.getElementById("hide-markers")
	if(hide_markers_elem)
	{
		hide_markers_elem.addEventListener("click", hideMarkers);
	}
	var delete_markers_elem = document.getElementById("delete-markers");
	if(delete_markers_elem)
	{
		delete_markers_elem.addEventListener("click", deleteMarkers);
	}
    

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
		hasBeenSaved = false;
		db_id = "";
		setId(newId){
			this.db_id = newId;
		}
		getId(){
			return this.db_id;
		}
		setHasBeenSaved(wasSaved){
			this.hasBeenSaved = wasSaved;
		}
		content_changed(){
			console.log("it changed");
		}

		init(){

			const divElem = document.createElement("div");
			divElem.setAttribute("class", "container");

			const textNode = document.createElement("textarea"); 
			textNode.setAttribute("class", "textArea");
			textNode.setAttribute("contenteditable", "true");
			textNode.placeholder = "Edit me!";
			
			divElem.appendChild(textNode);

			const saveButton = document.createElement('button');
			saveButton.setAttribute("class", "saveBtn");
			saveButton.textContent = 'Save';

			saveButton.addEventListener('click', () => {
				var noteData=textNode.value;
				console.log(noteData);
				var body = {data: noteData, position: pin_map.get(this.db_id).marker.position};
				if (this.hasBeenSaved) {
					body.db_id = this.db_id;
				}
				fetch('/api/pin', {
					method: 'POST',
					headers: {
						'Content-Type': 'application/json',
					},
					body: JSON.stringify(body),
				})
				.then(response => response.json())
				.then((data) => {
					if(!pin_map.has(data.db_id))
					{
						pin_map.set(data.db_id, pin_map.get(this.db_id));
						pin_map.delete(this.db_id);	
						this.db_id = data.db_id;
					}
					this.hasBeenSaved = true;
				})
				.catch((error) => {
					console.error('Error:', error);
				});
			});

			const deleteButton = document.createElement('button');
			deleteButton.setAttribute("class", "deleteBtn");
			deleteButton.textContent = 'Delete';

			deleteButton.addEventListener('click', () => {
				fetch(`/api/delete_pin/${this.db_id}`, {
					method: 'DELETE',
					headers: {
						'Content-Type': 'application/json',
					},
				})
				.then((data) => {
					console.log(data);
					pin_map.get(this.db_id).marker.setMap(null);
					pin_map.delete(this.db_id);
				})
				.catch((error) => {
					console.error('Error:', error);
				});
				});


			divElem.appendChild(saveButton);
			divElem.appendChild(deleteButton);
			this.setContent(divElem);
			this.myholytextelem = textNode;
		}
	}
	
	function placeMarker(location, db_id=fake_id) {
		
		const marker = new google.maps.Marker({
		position: location,
		icon: icons["yellow_pin"].icon,
		map: map,
		});

		const infowindow = new NoteWindow();
		infowindow.setId(db_id);
		infowindow.setHasBeenSaved(db_id != fake_id);
		infowindow.init();

		var pin = Object.create(PinStruct).initialize(marker, infowindow);
		console.log(`Created pin: ${db_id}`);
		pin_map.set(db_id, pin);

		pin_id++;

		marker.addListener("click", () => {
			infowindow.open({
			  anchor: marker,
			  map,
			});
			console.log(infowindow.getId());
			console.log(infowindow);
		});

		//double click to delete pin
		marker.addListener("dblclick", function() {
			marker.setMap(null);
			pin_map.delete(db_id);
		});

		return pin;

	}
	
	// Load user pins
	loadPins().then((data) => {
		console.log(data);
		data.forEach(element => {
			let newPin = placeMarker(element.position, element.db_id);
			newPin.infowindow.myholytextelem.textContent = element.data;
			newPin.infowindow.db_id = element.db_id;
		});
	});

	//Directions
	var defaultInfoWindow = new google.maps.InfoWindow();
	const directionsService = new google.maps.DirectionsService();
	const directionsRenderer = new google.maps.DirectionsRenderer({
		polylineOptions: {
			strokeColor: "#1E90FF",
		},
		infoWindow: defaultInfoWindow,
		draggable: true,
	  });
  
	directionsRenderer.setMap(map);
  
	document.addEventListener('keydown', function(event) {
		if (event.key === 'Enter') {
			calculateAndDisplayRoute(directionsService, directionsRenderer);
		}
	});
	
}


// Draw the array of boxes as polylines on the map
function drawBoxes(boxes){
	var boxpolys = new Array(boxes.length);
	for (var i = 0; i < boxes.length; i++) {
	  boxpolys[i] = new google.maps.Rectangle({
		bounds: boxes[i],
		fillOpacity: 0,
		strokeOpacity: 1.0,
		strokeColor: '#000000',
		strokeWeight: 1,
		map: map
	  });
	}
  };

function calculateAndDisplayRoute(directionsService, directionsRenderer) {
	directionsService
	  .route({
		origin: {
		  query: document.getElementById("start").value,
		},	
		destination: {
		  query: document.getElementById("end").value,
		},
		travelMode: google.maps.TravelMode.DRIVING,
	  })
	  .then((response) => {
		console.log("How about here?");

		//Use Route boxer to define bounding area to seach in for Places
		var route = response.routes[0];
		console.log("route:", route);
		var path = route.overview_path;

		console.log(path);

		var dist = route.legs[0].distance.value/ROUTE_DISTANCE_DIVISOR;

		console.log("dist: ", dist);

		console.log("route boxer still exists?", routeBoxer);
		
        boxes = routeBoxer.box(path, 10);

		console.log("Do we get here?");

		console.log(boxes);
		//Draw debug boxes
		drawBoxes(boxes);

		directionsRenderer.setDirections(response);
	  })
	  .catch((e) => window.alert("Directions request failed due to " + e));
  }



// Sets the map on all markers in the array.
function setMapOnAll(map) {
	for (let pair of pin_map.entries()){
		pair[1].marker.setMap(map);
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
	pin_map.clear();
  }

  // Burger menus (repurposed from https://tailwindcomponents.com/component/navbar-hamburger-menu)
  function burgerMenuActions() {
	// open
	const burger = document.querySelectorAll('.navbar-burger');
	const menu = document.querySelectorAll('.navbar-menu');

	if (burger.length && menu.length) {
		for (var i = 0; i < burger.length; i++) {
			burger[i].addEventListener('click', function() {
				for (var j = 0; j < menu.length; j++) {
					menu[j].classList.toggle('hidden');
				}
			});
		}
	}

	// close
	const close = document.querySelectorAll('.navbar-close');
	const backdrop = document.querySelectorAll('.navbar-backdrop');

	if (close.length) {
		for (var i = 0; i < close.length; i++) {
			close[i].addEventListener('click', function() {
				for (var j = 0; j < menu.length; j++) {
					menu[j].classList.toggle('hidden');
				}
			});
		}
	}

	if (backdrop.length) {
		for (var i = 0; i < backdrop.length; i++) {
			backdrop[i].addEventListener('click', function() {
				for (var j = 0; j < menu.length; j++) {
					menu[j].classList.toggle('hidden');
				}
			});
		}
	}
  }
window.initMap = initMap;
