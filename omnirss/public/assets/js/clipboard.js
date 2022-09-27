function copy_to_clipboard(url) {
    navigator.clipboard.writeText(url).then(function() {
        alert('Paste into your RSS client');
    }, function(err) {
        console.error('Could not copy to clipboard: ', err);
    });
}
