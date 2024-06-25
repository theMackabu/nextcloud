const dragRegionProperties = {
	zIndex: 1000,
	height: '18px',
	width: '100%',
	position: 'fixed',
	top: '0',
	left: '0'
};

const smallRegionProperties = {
	zIndex: 1000,
	height: '50px',
	width: '20%',
	position: 'fixed',
	top: '0',
	right: '16%'
};

document.addEventListener('DOMContentLoaded', () => {
	const styleElement = document.createElement('style');

	const cssRule = `
		html, body { background: transparent !important; }
		:root { --color-main-background: transparent !important; }
		#header { background-color: rgba(23,23,23,.6) !important; }
		.app-navigation { background-color: rgba(23,23,23,.3) !important; }
		.header-menu__wrapper { background-color: rgba(23,23,23,.5) !important; }
	`;

	styleElement.appendChild(document.createTextNode(cssRule));
	document.head.appendChild(styleElement);

	const dragRegion = document.createElement('div');
	const smallDragRegion = document.createElement('div');

	const logo = document.querySelector('.logo-icon');
	const anchor = document.getElementById('nextcloud');
	const header = document.querySelector('div.header-left');

	if (logo) {
		logo.parentNode.removeChild(logo);
	}
	if (anchor) {
		anchor.parentNode.removeChild(anchor);
	}
	if (header) {
		header.style.marginLeft = '100px';
	}

	Object.assign(dragRegion.style, dragRegionProperties);
	dragRegion.setAttribute('data-tauri-drag-region', '');
	document.documentElement.appendChild(dragRegion);

	Object.assign(smallDragRegion.style, smallRegionProperties);
	smallDragRegion.setAttribute('data-tauri-drag-region', '');
	document.documentElement.appendChild(smallDragRegion);
});
