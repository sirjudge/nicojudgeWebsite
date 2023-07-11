import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { solid, regular, brands } from '@fortawesome/fontawesome-svg-core/import.macro'


export default function Header(){
    return (
        <table>
            <tbody>
                <tr>
                    <td className="center">
                        <h1 className="header center">Nico Judge</h1>
                    </td>
                </tr>
                <tr>
                    <td className="center">
                        <h4 className="location">Full Stack Software Engineer</h4>
                    </td>
                </tr>
                <tr>
                    <td className="center">
                        <tr>
                            <td>
                                <a href="https://www.github.com/sirjudge" className="gitIcon"><FontAwesomeIcon icon={brands("github")}/></a>
                            </td>
                            <td>
                                <a href="mailto:nico.a.judge@gmail.com" className="emailIcon"><FontAwesomeIcon icon={solid("envelope")}/></a>
                            </td>
                            <td>
                                <a href="https://www.linkedin.com/in/nicojudge/"  className="linkedInIcon"><FontAwesomeIcon icon={brands("linkedin")}/></a>
                            </td>
                        </tr>
                    </td>
                </tr>
            </tbody>
        </table>

    )
}