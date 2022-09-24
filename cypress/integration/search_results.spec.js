describe('Search Results', () => {
    beforeEach(() => {
        cy.visit('/.netlify/functions/search?query=Joe+Rogan');
    })

    it('Copy to clipboard', () => {
        cy.get('button')
            .contains('Copy RSS Feed to Clipboard')
            .first()
            .click();

        cy.window()
            .its('navigator.clipboard')
            .invoke('readText')
            .should('contain', '/.netlify/functions/spotify-rss?id=4rOoJ6Egrf8K2IrywzwOMk');
    })
})
