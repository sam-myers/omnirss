describe('Home Page', () => {
    beforeEach(() => {
        cy.visit('/')
    })

    it('Navbar contains OmniRSS', () => {
        cy.get('.navbar')
            .contains('OmniRSS');
    })

    it('Feature - Bring Your Own Client', () => {
        cy.get('.features-icons')
            .contains('Bring Your Own Client');
    })

    it('Feature - Private', () => {
        cy.get('.features-icons')
            .contains('Private');
    })

    it('Feature - Open Source', () => {
        cy.get('.features-icons')
            .contains('Open Source');
    })

    it('Link to Source Code', () => {
        cy.get('.features-icons p a')
            .contains('Source Code')
            .should('have.attr', 'href', 'https://github.com/sam-myers/omnirss/');
    })

    it('Made in USA', () => {
        cy.get('.footer')
            .contains('Proudly made in the USA.');
    })
})
