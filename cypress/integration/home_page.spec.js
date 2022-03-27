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

    it('Feature - Free Forever', () => {
        cy.get('.features-icons')
            .contains('Free Forever');
    })

    it('Feature - Private', () => {
        cy.get('.features-icons')
            .contains('Private');
    })

    it('Made in USA', () => {
        cy.get('.footer')
            .contains('Proudly made in the USA.');
    })
})
