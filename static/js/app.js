document.addEventListener('DOMContentLoaded', () => {
    const menuButton = document.querySelector('button');
    const menuDropdown = menuButton.nextElementSibling;

    menuButton.addEventListener('click', () => {
        menuDropdown.classList.toggle('hidden');
    });

    // Close menu when clicking outside
    document.addEventListener('click', (event) => {
        if (!menuButton.contains(event.target) && !menuDropdown.contains(event.target)) {
            menuDropdown.classList.add('hidden');
        }
    });
});

function handleLogout() {
    window.location.href = '/admin/logout';
}
