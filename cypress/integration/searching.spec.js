describe('Searching', () => {
    beforeEach(() => {
        cy.visit('/');
    })

    it('Type -> Enter', () => {
        cy.get('input[name*="search"]')
            .type('Joe Rogan{enter}');

        cy.url().should('include', '/?search=Joe+Rogan');

        cy.contains("The Joe Rogan Experience");
        cy.contains("The official podcast of comedian Joe Rogan.");
    })

    it('Type -> Click Search', () => {
        cy.get('input')
            .type('Joe Rogan');

        cy.get('button[type*="submit"]').click();

        cy.url().should('include', '/?search=Joe+Rogan');

        cy.contains("The Joe Rogan Experience");
        cy.contains("The official podcast of comedian Joe Rogan.");
    })
})
