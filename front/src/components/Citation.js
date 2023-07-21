import { Container } from 'react-bootstrap';

// Text for 'Citation' section
const Citation = () => { 
    
    return ( 
        <Container>
            <h1>Citation</h1>
            <cite>
                Kakoulidis, P., Vlachos, I.S., Thanos, D. et al. Identifying and profiling structural similarities between 
                Spike of SARS-CoV-2 and other viral or host proteins with Machaon. Commun Biol 6, 752 (2023):  <a 
                target="_blank" rel="noopener noreferrer" href='https://doi.org/10.1038/s42003-023-05076-7'>https://doi.org/10.1038/s42003-023-05076-7</a>
            </cite>
        </Container>
    )    
}

export default Citation;
