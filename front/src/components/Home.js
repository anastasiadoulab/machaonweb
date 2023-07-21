import RequestForm from "./RequestForm";

// root component for 'Home' section 
const Home = () => {

      
    return (
        <>
            <div className="logo-container">
                <div className="logo-title">
                    Machaon<span className="logo-suffix">Web</span>
                </div>
                <div className="logo-footer">
                    Identify & profile proteins with similar structures
                </div>
            </div>
            <RequestForm />
        </>
    )    
}

export default Home;
