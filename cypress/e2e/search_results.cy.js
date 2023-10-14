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

    it('Check status code and XML validity', () => {
        cy.get('button')
            .contains('Copy RSS Feed to Clipboard')
            .first()
            .click();

        cy.window()
            .its('navigator.clipboard')
            .invoke('readText')
            .then((clipboardText) => {
                cy.request(clipboardText)
                    .its('status')
                    .should('eq', 200);

                cy.request(clipboardText)
                    .its('headers')
                    .its('content-type')
                    .should('include', 'application/xml');
            });
    })

    it('Check navigation to home page', () => {
        cy.visit('/.netlify/functions/search?query=Joe+Rogan');
        cy.get('.navbar-brand').click();
        cy.url().should('eq', Cypress.config().baseUrl + '/');
    })
})
