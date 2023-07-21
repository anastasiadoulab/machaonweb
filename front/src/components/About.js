import { Container } from 'react-bootstrap';
import { Link } from "react-router-dom";

// Text for 'About' section
const About = () => {
      
    return ( 
        <Container className="about-section">
            <h1>About</h1>
            <h3>The project</h3>
            This work is part of the PhD thesis of <a href="https://gr.linkedin.com/in/panoskakoulidis">Panos Kakoulidis</a>, graduate student 
            of Department of Informatics and Telecommunications, National and Kapodistrian University of Athens (<a href="https://www.di.uoa.gr/en">NKUA</a>) and Biomedical Research Foundation of the Academy of Athens (<a href="http://www.bioacademy.gr/?lang=en">BRFAA</a>). He designed, 
            implemented, tested, deployed and formally documented this method under the supervision of <b>Dr. Ioannis S. Vlachos</b> (Broad Institute of MIT and 
            Harvard, Cambridge, MA, USA | Department of Pathology, Beth Israel Deaconess Medical Center, Boston, MA, USA | Harvard Medical School, Boston, 
            MA, USA | Spatial Technologies Unit, Harvard Medical School Initiative for RNA Medicine, Boston, MA, USA), <b>Prof. Dimitris Thanos</b> (Biomedical 
            Research Foundation, Academy of Athens, Athens, Greece), <b>Prof. Gregory L. Blatch</b> (Higher Colleges of Technology, UAE), <b>Prof. Ioannis Z. Emiris</b> (ATHENA Research and Innovation Center, Maroussi, Greece | 
            Department of Informatics and Telecommunications, National and Kapodistrian University of Athens, Athens, Greece) and <b>Dr. Ema Anastasiadou </b> 
             (Biomedical Research Foundation, Academy of Athens, Athens, Greece).
            <br/><br/>
            The project is self-funded and there are no conflicts of interest. Publication fees are covered by Higher Colleges of Technology.
            <br/><br/>
            <h3>The method</h3>
            <b>Machaon</b> identifies similar protein structures and profiles these structural relationships by meta-analysis. The core metrics for the 
            comparisons are b-phipsi, r-dist and t-alpha that refer to the torsion angles, the inter-residue distances and the surface complexity of a protein 
            structure, These metrics can operate in whole structure, domain or segment  level and they do not rely on any kind of alignment for their 
            computations. The method is not only focused to  find long, contiguous and highly similar structures but it is also fuzzy enough to detect 
            dispersed similarities that could offer hints for novel properties and hidden relationships. Meta-analysis extends the comparisons on the 
            identified similar structures with established metrics, joins metadata and genomic information with the results and correlates specific 
            areas of the protein with biological properties.   Therefore, Machaon is a multiomics method, primarily relying on structural data. For more, 
            please consult the <Link to="/citation">manuscript</Link>. 
            <br/><br/>
            Machaon is implemented as an open source software and is written in Python programming language.  A thorough documentation and the code is 
            available at <a href="https://github.com/anastasiadoulab/machaon">Github</a>. Machaon is a command-line application that is easily accessible
            to every platform (Windows, Unix, MacOS) with Docker or Singularity. Alternatively, there is a gRPC module that allows external systems to 
            interface with Machaon. The application accepts structures from <a href="https://www.rcsb.org/">RCSB PDB</a>, <a href="https://alphafold.ebi.ac.uk">
            AlphaFold DB</a> and <a href="https://esmatlas.com">ESM Metagenomic Atlas</a>. It interfaces with <a href="https://www.ebi.ac.uk/QuickGO">EBI-QuickGO</a>, RCSB PDB, <a href="https://www.ncbi.nlm.nih.gov/Web/Search/entrezfs.html">NCBI Entrez</a>, <a 
            href="https://www.ncbi.nlm.nih.gov/refseq/">NCBI RefSeq</a> and <a href="https://www.uniprot.org/">UniProt</a> for meta-analysis. The implementation 
            respects third-party data providers as it is focused on minimum intercommunication via offline data and distributed caching mechanism. 
            <br/><br/>
            <br/>
            <h3>The online system</h3>
            <b>MachaonWeb</b> is a distributed and scalable computing network that offers an alternative access to Machaon for the community via the Web. The backend is written in 
            Rust programming language, securely employing REST and mTLS-based gRPC protocols. The interface is implemented with React and Bootstrap frameworks. MachaonWeb is open source software and 
            is containerized with Docker/Singularity for easy deployment. You can find the code at  <a 
            href="https://github.com/anastasiadoulab/machaonweb">Github</a>. Thus, anyone can deploy the method online wherever/whenever is convenient and 
            customize it according to the current needs, regardless of the current deployment's status.

            The network is currently hosted in <a href="https://hypatia.athenarc.gr/">Hypatia</a> cloud infrastructure by 
            <a href="https://www.athenarc.gr/en"> Athena RC</a> and <a href="https://grnet.gr/en/">GRNET</a>,  under the <a href="https://elixir-greece.org/">Elixir program</a>. 
            <br/><br/>
     
        </Container>
    )    
}

export default About;